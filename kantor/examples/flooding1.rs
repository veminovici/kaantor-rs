use actix::prelude::*;
use kantor::{node::NodeActor, *};
use log::debug;

enum MyPayload {
    Position(usize),
}

struct MyHandler {
    aid: ActorId,
    count: usize,
}

impl MyHandler {
    pub fn new(aid: ActorId) -> Self {
        Self { aid, count: 0 }
    }
}

impl ProtocolHandler for MyHandler {
    type Payload = MyPayload;

    fn aid(&self) -> ActorId {
        self.aid
    }

    fn receive(
        &mut self,
        _proxies: &mut Proxies<Self::Payload>,
        msg: protocol::Message<Self::Payload>,
    ) {
        self.count += 1;
        println!("Actor {:?} received a protocol {:?} message", self.aid, msg);
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

        let msg = p1
            .protocol_builder()
            .to_actor(*p2.aid())
            .with_payload(MyPayload::Position(10))
            .with_hid(*p1.aid())
            .build();

        let _ = p1.send(msg).await;
    });

    println!("Finished the test");
    debug!("Completed the example FLOODING");
}
