use actix::prelude::*;
use kantor::{node::NodeActor, protocol::{Builder, SessionId}, *};
use log::debug;
#[derive(Clone)]
enum MyPayload {
    Start(usize),
    Forward(usize),
}

struct MyHandler {
    aid: ActorId,
    sessions: Vec<SessionId>,
}

impl MyHandler {
    pub fn build(aid: ActorId) -> Node<NodeActor<Self>, MyPayload> {
        let h = Self {
            aid,
            sessions: vec![]
        };
        NodeActor::build(h)
    }
}

impl ProtocolHandler for MyHandler {
    type Payload = MyPayload;

    fn aid(&self) -> ActorId {
        self.aid
    }

    fn receive(
        &mut self,
        proxies: &mut Proxies<Self::Payload>,
        msg: protocol::Message<Self::Payload>,
    ) {
        println!("Actor {:?} received a protocol {:?} message", self.aid, msg);

        let payload = msg.payload();

        match payload {
            MyPayload::Start(value) => {
                let sid = msg.sid();
                self.sessions.push(*sid);

                let msg = Builder::with_from_to(&msg)
                    .with_session(*sid)
                    .with_payload(MyPayload::Forward(*value))
                    .with_hid(self.aid)
                    .build();
                proxies.do_send_all_except(&self.aid, msg, &[])
            }
            MyPayload::Forward(_value) => {
                let sid = msg.sid();

                if !self.sessions.contains(sid) {
                    let hid: ActorId = msg.hid().aid();
                    // forward the message
                    let msg = Builder::with_message(msg).with_hid(self.aid).build();
                    proxies.do_send_all_except(&self.aid, msg, &[hid])
                } else {
                    debug!("Received a message for a recorded sessions {:?}", sid)
                }
            }
        }
    }
}

fn main() {
    env_logger::init();
    debug!("Starting the example FLOODING");

    let sys = System::new();
    sys.block_on(async {
        let mut p1 = MyHandler::build(1.into());
        let mut p2 = MyHandler::build(2.into());
        let mut p3 = MyHandler::build(3.into());

        add_edge(&mut p1, &mut p2).await; // 1 - 2
        add_edge(&mut p1, &mut p3).await; // 1 - 3

        let msg = Builder::with_from(*p1.aid())
            .with_to_actor(*p2.aid())
            .with_session(50.into())
            .with_payload(MyPayload::Start(999))
            .with_hid(*p1.aid())
            .build();

        let _ = p1.send(msg).await;
    });

    println!("Finished the test");
    debug!("Completed the example FLOODING");
}
