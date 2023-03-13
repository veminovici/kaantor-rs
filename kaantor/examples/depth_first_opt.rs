use actix::prelude::*;
use kaantor::{
    protocol::{Builder, Session},
    NodeActor, *,
};
use log::{debug, info};
use std::fmt::Debug;

// use ptree::*;

#[derive(Clone)]
enum Payload {
    Start,
    Go(Vec<ActorId>),
    Back(Vec<ActorId>),
}

impl Debug for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "START"),
            Self::Go(visited) => {
                let visited = debug_iter(visited.iter());
                write!(f, "GO | {}", visited)
            }
            Self::Back(visited) => {
                let visited = debug_iter(visited.iter());
                write!(f, "BACK | {}", visited)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Parent {
    NoParent,
    Root,
    Parent(ActorId),
}

trait DFHandler {
    fn handle_start(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: &mut impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload>;

    fn handle_go(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
        visited: Vec<ActorId>,
    ) -> ContinuationHandler<Payload>;

    fn handle_back(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
        visited: Vec<ActorId>,
    ) -> ContinuationHandler<Payload>;
}

trait DFSender {
    fn send_go_to_node(
        &self,
        sid: Session,
        to: ActorId,
        visited: Vec<ActorId>,
    ) -> ContinuationHandler<Payload>;
    fn send_back_to_node(
        &self,
        sid: Session,
        to: ActorId,
        visited: Vec<ActorId>,
    ) -> ContinuationHandler<Payload>;
}

struct Handler {
    aid: ActorId,
    parent: Parent,
    children: Vec<ActorId>,
}

impl DFHandler for Handler {
    fn handle_start(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: &mut impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.parent = Parent::Root;

        match ns.next() {
            Some(to) => {
                self.children.push(to);
                let visited = vec![self.aid];

                self.send_go_to_node(*msg.session(), to, visited)
            }
            None => {
                self.debug_spanning_node();
                ContinuationHandler::Done
            }
        }
    }

    fn handle_go<'a>(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
        visited: Vec<ActorId>,
    ) -> ContinuationHandler<Payload> {
        assert!(self.parent == Parent::NoParent);
        self.parent = Parent::Parent(msg.sender().as_aid());

        match ns.into_iter().find(|n| !visited.contains(n)) {
            Some(to) => {
                self.children = vec![to];

                let mut visited = visited.clone();
                visited.push(self.aid);

                self.send_go_to_node(*msg.session(), to, visited)
            }
            None => {
                self.children = vec![];

                let mut visited = visited.clone();
                visited.push(self.aid);

                self.send_back_to_node(*msg.session(), msg.sender().as_aid(), visited)
            }
        }
    }

    fn handle_back(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
        visited: Vec<ActorId>,
    ) -> ContinuationHandler<Payload> {
        match ns.into_iter().find(|n| !visited.contains(n)) {
            Some(to) => {
                self.children.push(to);

                self.send_go_to_node(*msg.session(), to, vec![])
            }
            None => match self.parent {
                Parent::NoParent => panic!("We should not be here"),
                Parent::Root => {
                    info!("Finished the spanning tree");
                    ContinuationHandler::Done
                }
                Parent::Parent(pid) => self.send_back_to_node(*msg.session(), pid, visited),
            },
        }
    }
}

impl DFSender for Handler {
    fn send_go_to_node(
        &self,
        sid: Session,
        to: ActorId,
        visited: Vec<ActorId>,
    ) -> ContinuationHandler<Payload> {
        let msg = Builder::with_from_actor(self.aid)
            .with_to_actor(to)
            .with_session(sid)
            .with_payload(Payload::Go(visited))
            .with_sender(self.aid)
            .build();

        ContinuationHandler::SendToNode(to, msg)
    }

    fn send_back_to_node(
        &self,
        sid: Session,
        to: ActorId,
        visited: Vec<ActorId>,
    ) -> ContinuationHandler<Payload> {
        let msg = Builder::with_from_actor(self.aid)
            .with_to_actor(to)
            .with_session(sid)
            .with_payload(Payload::Back(visited))
            .with_sender(self.aid)
            .build();

        ContinuationHandler::SendToNode(to, msg)
    }
}

impl Handler {
    /// Builds a handler
    fn build(aid: ActorId) -> NodeHandler<Self> {
        NodeActor::build(Self {
            aid,
            parent: Parent::NoParent,
            children: vec![],
        })
    }

    #[inline]
    fn debug_spanning_node(&self) {
        info!(
            "SPANNING-TREE NODE: {:?} p={:?} cs={:?}",
            self.aid, self.parent, self.children
        );
    }
}

impl ProtocolHandler for Handler {
    type Payload = Payload;

    fn aid(&self) -> ActorId {
        self.aid
    }

    fn receive(
        &mut self,
        ns: impl Iterator<Item = ActorId>,
        msg: protocol::Message<Self::Payload>,
    ) -> ContinuationHandler<Self::Payload> {
        match &msg.payload() {
            &Payload::Start => self.handle_start(&msg, &mut ns.into_iter()),
            &Payload::Go(visited) => self.handle_go(&msg, ns, visited.clone()),
            &Payload::Back(visited) => self.handle_back(&msg, ns, visited.clone()),
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
        let mut p4 = Handler::build(4.into());
        let mut p5 = Handler::build(5.into());

        add_edge(&mut p1, &mut p2).await; // 1 - 2
        add_edge(&mut p1, &mut p3).await; // 1 - 3
        add_edge(&mut p2, &mut p4).await; // 2 - 4
        add_edge(&mut p4, &mut p5).await; // 4 - 5
        add_edge(&mut p3, &mut p5).await; // 3 - 5

        // Start the flooding, the idea is to prapagate to all nodes
        // the payload 999, starting with the first node.

        let msg = Builder::with_from_api()
            .with_to_actor(p1.aid())
            .with_session(50.into())
            .with_payload(Payload::Start)
            .with_sender(p1.aid())
            .build();

        let _ = p1.send(msg).await;
    });

    println!("Finished the test");
}
