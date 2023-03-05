use actix::prelude::*;
use kantor::{
    node::{Builder as NBuilder, Player, PlayerHandler},
    protocol::Message as ProMsg,
    *,
};
use log::debug;

enum MyPayload {
    Position(usize),
}

struct MyProtocol {
    aid: ActorId,
}

impl MyProtocol {
    pub fn new(aid: ActorId) -> Self {
        Self { aid }
    }
}

impl PlayerHandler for MyProtocol {
    type Payload = MyPayload;

    fn aid(&self) -> ActorId {
        self.aid
    }

    fn handler(
        &mut self,
        proxies: &mut Proxies<Self::Payload>,
        msg: protocol::Message<Self::Payload>,
    ) {
        println!("Actor {:?} received a protocol {:?} message", self.aid, msg);
    }
}

fn main() {
    env_logger::init();
    debug!("Starting the example FLOODING");

    let sys = System::new();
    sys.block_on(async {
        let myProtocol1 = MyProtocol::new(1.into());
        let myProtocol2 = MyProtocol::new(2.into());
        let myProtocol3 = MyProtocol::new(3.into());

        let mut p1 = Player::build(myProtocol1);
        let mut p2 = Player::build(myProtocol2);
        let mut p3 = Player::build(myProtocol3);

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
