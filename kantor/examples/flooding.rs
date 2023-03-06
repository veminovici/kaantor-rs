use actix::prelude::*;
use kantor::{node::NodeActor, protocol::Builder, *};
use log::debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SessionId(usize);

impl From<usize> for SessionId {
    fn from(value: usize) -> Self {
        SessionId(value)
    }
}

#[derive(Clone)]
enum MyPayload {
    Start(SessionId, usize),
    Forward(SessionId, usize),
}

struct MyHandler {
    aid: ActorId,
    sessions: Vec<SessionId>,
}

impl MyHandler {
    pub fn new(aid: ActorId) -> Self {
        Self {
            aid,
            sessions: Default::default(),
        }
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
            MyPayload::Start(sid, value) => {
                self.sessions.push(*sid);

                let msg = Builder::with_from_to(&msg)
                    .with_payload(MyPayload::Forward(*sid, *value))
                    .with_hid(self.aid)
                    .build();
                proxies.do_send_all_except(&self.aid, msg, &[])
            }
            MyPayload::Forward(sid, _value) => {
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
        let handler1 = MyHandler::new(1.into());
        let handler2 = MyHandler::new(2.into());
        let handler3 = MyHandler::new(3.into());

        let mut p1 = NodeActor::build(handler1);
        let mut p2 = NodeActor::build(handler2);
        let mut p3 = NodeActor::build(handler3);

        add_edge(&mut p1, &mut p2).await; // 1 - 2
        add_edge(&mut p1, &mut p3).await; // 1 - 3

        let msg = Builder::with_from(*p1.aid())
            .with_to_actor(*p2.aid())
            .with_payload(MyPayload::Start(10.into(), 299))
            .with_hid(*p1.aid())
            .build();

        let _ = p1.send(msg).await;
    });

    println!("Finished the test");
    debug!("Completed the example FLOODING");
}
