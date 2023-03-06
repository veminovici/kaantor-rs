use actix::prelude::*;
use kantor::{
    protocol::Builder,
    NodeActor, *,
};
use log::{debug, info};

#[derive(Debug, Clone)]
enum Payload {
    Start,
    Go,
    BackChild,
    BackNoChild,
}

enum Parent {
    NoParent,
    Root,
    Parent(ActorId),
}

struct Handler {
    aid: ActorId,
    parent: Parent,
    children: Vec<ActorId>,
    exp_messages: usize,
}

impl Handler {
    pub fn build(aid: ActorId) -> Node<NodeActor<Self>, Payload> {
        NodeActor::build(Self {
            aid,
            parent: Parent::NoParent,
            children: vec![],
            exp_messages: 0,
        })
    }
}

impl ProtocolHandler for Handler {
    type Payload = Payload;

    fn aid(&self) -> ActorId {
        self.aid
    }

    fn receive(
        &mut self,
        proxies: &Proxies<Self::Payload>,
        msg: protocol::Message<Self::Payload>,
    ) -> ContinuationHandler<Self::Payload> {
        match msg.payload() {
            &Payload::Start => {
                let sid = msg.sid();
                self.parent = Parent::Root;
                self.exp_messages = proxies.aids().count();

                let msg = Builder::with_from_actor(self.aid)
                    .with_to_all_actors()
                    .with_session(*sid)
                    .with_payload(Payload::Go)
                    .with_hid(self.aid)
                    .build();

                ContinuationHandler::SendToAllNodes(msg)
            }
            &Payload::Go => {
                let sid = msg.sid();

                match self.parent {
                    Parent::NoParent => {
                        self.parent = Parent::Parent(msg.hid().aid());
                        self.exp_messages = proxies.aids().count() - 1;
                        let hid = msg.hid().aid();

                        if self.exp_messages != 0 {
                            let msg = Builder::with_from_actor(self.aid)
                                .with_to_all_actors()
                                .with_session(sid.clone())
                                .with_payload(Payload::Go)
                                .with_hid(self.aid)
                                .build();

                            ContinuationHandler::SendToAllNodesExcept(msg, vec![hid])
                        } else {
                            let msg = Builder::with_from_actor(self.aid)
                                .with_to_actor(hid)
                                .with_session(sid.clone())
                                .with_payload(Payload::BackChild)
                                .with_hid(self.aid)
                                .build();

                            ContinuationHandler::SendToNode(hid, msg)
                        }
                    }
                    Parent::Root => {
                        let hid = msg.hid().aid();

                        let msg = Builder::with_from_actor(self.aid)
                            .with_to_all_actors()
                            .with_session(sid.clone())
                            .with_payload(Payload::BackNoChild)
                            .with_hid(self.aid)
                            .build();

                        ContinuationHandler::SendToNode(hid, msg)
                    }
                    Parent::Parent(_) => {
                        let hid = msg.hid().aid();
                        let msg = Builder::with_from_actor(self.aid)
                            .with_to_all_actors()
                            .with_session(sid.clone())
                            .with_payload(Payload::BackNoChild)
                            .with_hid(self.aid)
                            .build();

                        ContinuationHandler::SendToNode(hid, msg)
                    }
                }
            }
            &Payload::BackNoChild => {
                self.exp_messages -= 1;
                let sid = msg.sid();

                if self.exp_messages == 0 {
                    match self.parent {
                        Parent::NoParent => panic!("We shoud not be here"),
                        Parent::Root => {
                            info!("Finished the spanning tree");
                            ContinuationHandler::Done
                        }
                        Parent::Parent(_) => {
                            let hid = msg.hid().aid();
                            let msg = Builder::with_from_actor(self.aid)
                                .with_to_all_actors()
                                .with_session(sid.clone())
                                .with_payload(Payload::BackChild)
                                .with_hid(self.aid)
                                .build();
                            ContinuationHandler::SendToNode(hid, msg)
                        }
                    }
                } else {
                    ContinuationHandler::Done
                }
            }
            &Payload::BackChild => {
                self.exp_messages -= 1;
                self.children.push(msg.hid().aid());
                let sid = msg.sid();

                if self.exp_messages == 0 {
                    match self.parent {
                        Parent::NoParent => panic!("We shoud not be here"),
                        Parent::Root => {
                            info!("Finished the spanning tree");
                            ContinuationHandler::Done
                        }
                        Parent::Parent(_) => {
                            let hid = msg.hid().aid();
                            let msg = Builder::with_from_actor(self.aid)
                                .with_to_all_actors()
                                .with_session(sid.clone())
                                .with_payload(Payload::BackChild)
                                .with_hid(self.aid)
                                .build();
                            ContinuationHandler::SendToNode(hid, msg)
                        }
                    }
                } else {
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

        let mut p1 = Handler::build(1.into());
        let mut p2 = Handler::build(2.into());
        let mut p3 = Handler::build(3.into());
        // let mut p4 = Handler::build(4.into());
        // let mut p5 = Handler::build(5.into());

        add_edge(&mut p1, &mut p2).await; // 1 - 2
        add_edge(&mut p1, &mut p3).await; // 1 - 3
                                          // add_edge(&mut p2, &mut p4).await; // 2 - 4
                                          // add_edge(&mut p4, &mut p5).await; // 4 - 5
                                          // add_edge(&mut p3, &mut p5).await; // 3 - 5

        // Start the flooding, the idea is to prapagate to all nodes
        // the payload 999, starting with the first node.

        let msg = Builder::with_from_api()
            .with_to_actor(*p1.aid())
            .with_session(50.into())
            .with_payload(Payload::Start)
            .with_hid(*p1.aid())
            .build();

        let _ = p1.send(msg).await;
    });

    println!("Finished the test");
}
