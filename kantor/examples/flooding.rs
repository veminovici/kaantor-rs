use actix::prelude::*;
use kantor::{node::Builder as NBuilder, protocol::Message as ProMsg, *};

#[derive(Debug, Message, Clone, Copy)]
#[rtype(result = "()")]
enum MyMessage {
    Position(usize),
    //Forward(ActorId, usize),
}

type PMsg = ProMsg<MyMessage>;
type GMsg = graph::GraphMsg<PMsg>;

struct MyActor {
    aid: ActorId,
    proxies: Proxies<MyMessage>,
}

impl MyActor {
    pub fn new(aid: ActorId) -> Self {
        Self {
            aid,
            proxies: Default::default(),
        }
    }
}

impl Actor for MyActor {
    type Context = ::actix::Context<Self>;
}

impl MyActor {
    pub fn build(aid: ActorId) -> Node<MyActor, MyMessage> {
        let actor = MyActor::new(aid);
        let addr = MyActor::start(actor);

        NBuilder::from_aid(aid).with_addr(addr).build()
    }
}

impl Handler<GMsg> for MyActor {
    type Result = ();

    fn handle(&mut self, msg: GMsg, _ctx: &mut Self::Context) -> Self::Result {
        println!("Actor {:?} received a graph {:?} message", self.aid, msg);
        self.proxies.handle_msg(msg);
    }
}

impl Handler<PMsg> for MyActor {
    type Result = ();

    fn handle(&mut self, msg: PMsg, _: &mut Context<Self>) {
        println!("Actor {:?} received a protocol {:?} message", self.aid, msg);
    }
}

fn main() {
    env_logger::init();

    let sys = System::new();
    sys.block_on(async {
        let mut p1 = MyActor::build(1.into());
        let mut p2 = MyActor::build(2.into());
        let mut p3 = MyActor::build(3.into());

        add_edge(&mut p1, &mut p2).await; // 1 - 2
        add_edge(&mut p1, &mut p3).await; // 1 - 3

        let msg = p1
            .protocol_builder()
            .to_actor(*p2.aid())
            .with_payload(MyMessage::Position(10))
            .with_hid(*p1.aid())
            .build();

        let _ = p1.send(msg).await;
    });

    println!("Finished the test")
}
