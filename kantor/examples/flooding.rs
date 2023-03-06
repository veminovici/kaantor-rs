use actix::prelude::*;
use kantor::{
    node::NodeActor,
    protocol::{Builder, SessionId},
    *,
};
use log::{debug, info};

#[derive(Debug, Clone)]
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
            sessions: vec![],
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
        _proxies: &Proxies<Self::Payload>,
        msg: protocol::Message<Self::Payload>,
    ) -> ContinuationHandler<Self::Payload> {
        println!("Actor {:?} received a protocol {:?} message", self.aid, msg);

        let payload = msg.payload();

        match payload {
            MyPayload::Start(value) => {
                let sid = msg.sid();
                self.sessions.push(*sid);

                info!("Node {} received the payload", self.aid);

                let msg = Builder::with_from_actor(self.aid)
                    .with_to_all_actors()
                    .with_session(*sid)
                    .with_payload(MyPayload::Forward(*value))
                    .with_hid(self.aid)
                    .build();

                    ContinuationHandler::SendToAllNodes(self.aid, msg)
            }
            MyPayload::Forward(_value) => {
                let sid = msg.sid();

                if !self.sessions.contains(sid) {
                    info!("Node {} received the payload", self.aid);
                    self.sessions.push(sid.clone());

                    // forward the message to all neighbours excepts the source.
                    let hid: ActorId = msg.hid().aid();
                    let msg = Builder::with_message(msg).with_hid(self.aid).build();
                    ContinuationHandler::SendToAllNodesExcept(self.aid, msg, vec![hid])
                } else {
                    debug!("Received a message for a recorded sessions {:?}", sid);
                    ContinuationHandler::Done
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
        // Create the communication graph between the nodes
        // First we create the nodes and then we add the edges between them

        let mut p1 = MyHandler::build(1.into());
        let mut p2 = MyHandler::build(2.into());
        let mut p3 = MyHandler::build(3.into());
        let mut p4 = MyHandler::build(4.into());
        let mut p5 = MyHandler::build(5.into());

        add_edge(&mut p1, &mut p2).await; // 1 - 2
        add_edge(&mut p1, &mut p3).await; // 1 - 3
        add_edge(&mut p2, &mut p4).await; // 2 - 4
        add_edge(&mut p4, &mut p5).await; // 4 - 5
        add_edge(&mut p3, &mut p5).await; // 3 - 5

        // Start the flooding, the idea is to prapagate to all nodes
        // the payload 999, starting with the first node.

        let msg = Builder::with_from_api()
            .with_to_actor(*p1.aid())
            .with_session(50.into())
            .with_payload(MyPayload::Start(999))
            .with_hid(*p1.aid())
            .build();

        let _ = p1.send(msg).await;
    });

    println!("Finished the test");
    debug!("Completed the example FLOODING");
}
