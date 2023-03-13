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
    Go,
    BackChild,
    BackNotAChild,
}

impl Debug for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "START"),
            Self::Go => write!(f, "GO"),
            Self::BackChild => write!(f, "BACK CHILD"),
            Self::BackNotAChild => write!(f, "BACK NOT A CHILD"),
        }
    }
}

#[derive(Debug)]
enum Parent {
    NoParent,
    Root,
    Parent(ActorId),
}

trait DFHandler {
    fn handle_start(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload>;

    fn handle_go(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload>;

    fn handle_back_child(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload>;

    fn handle_back_not_a_child(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload>;
}

trait DFSender {
    fn send_go_to_node(&self, sid: Session, to: ActorId) -> ContinuationHandler<Payload>;

    fn send_back_child(&self, sid: Session, pid: ActorId) -> ContinuationHandler<Payload>;

    fn send_back_no_child(&self, sid: Session, to: ActorId) -> ContinuationHandler<Payload>;
}

struct Handler {
    aid: ActorId,
    parent: Parent,
    children: Vec<ActorId>,
    visited: Vec<ActorId>,
}

impl DFHandler for Handler {
    fn handle_start(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.parent = Parent::Root;

        match ns.into_iter().find(|n| !self.visited.contains(n)) {
            Some(to) => self.send_go_to_node(*msg.session(), to),
            None => self.handle_node_done(msg),
        }
    }

    fn handle_go(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        match self.parent {
            Parent::NoParent => {
                let sender = msg.sender().as_aid();
                self.parent = Parent::Parent(sender);
                self.visited.push(sender);

                match ns.into_iter().find(|n| !self.visited.contains(n)) {
                    Some(to) => self.send_go_to_node(*msg.session(), to),
                    None => self.handle_node_done(msg),
                }
            }
            Parent::Root => {
                // send a BACK(no) to the sender.
                self.send_back_no_child(*msg.session(), msg.sender().as_aid())
            }
            Parent::Parent(_) => self.send_back_no_child(*msg.session(), msg.sender().as_aid()),
        }
    }

    #[inline]
    fn handle_back_child(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.children.push(msg.sender().as_aid());
        self.handle_back(msg, ns)
    }

    #[inline]
    fn handle_back_not_a_child(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.handle_back(msg, ns)
    }
}

impl DFSender for Handler {
    #[inline]
    fn send_go_to_node(&self, sid: Session, to: ActorId) -> ContinuationHandler<Payload> {
        let msg = Builder::with_from_actor(self.aid)
            .with_to_actor(to)
            .with_session(sid)
            .with_payload(Payload::Go)
            .with_sender(self.aid)
            .build();

        ContinuationHandler::SendToNode(to, msg)
    }

    #[inline]
    fn send_back_child(&self, sid: Session, pid: ActorId) -> ContinuationHandler<Payload> {
        let msg = Builder::with_from_actor(self.aid)
            .with_to_actor(pid)
            .with_session(sid)
            .with_payload(Payload::BackChild)
            .with_sender(self.aid)
            .build();

        ContinuationHandler::SendToNode(pid, msg)
    }

    #[inline]
    fn send_back_no_child(&self, sid: Session, to: ActorId) -> ContinuationHandler<Payload> {
        let msg = Builder::with_from_actor(self.aid)
            .with_to_actor(to)
            .with_session(sid)
            .with_payload(Payload::BackNotAChild)
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
            visited: vec![],
        })
    }

    #[inline]
    fn debug_spanning_node(&self) {
        info!(
            "SPANNING-TREE NODE: {:?} p={:?} cs={:?}",
            self.aid, self.parent, self.children
        );
    }

    fn handle_node_done(
        &mut self,
        msg: &protocol::Message<Payload>,
    ) -> ContinuationHandler<Payload> {
        self.debug_spanning_node();

        match self.parent {
            Parent::NoParent => panic!("We should not be here!"),
            Parent::Root => {
                info!("Finished the spanning tree");
                ContinuationHandler::Done
            }
            Parent::Parent(pid) => self.send_back_child(*msg.session(), pid),
        }
    }

    fn handle_back(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.visited.push(msg.sender().as_aid());

        match ns.into_iter().find(|n| !self.visited.contains(n)) {
            Some(to) => self.send_go_to_node(*msg.session(), to),
            None => self.handle_node_done(msg),
        }
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
        match msg.payload() {
            &Payload::Start => self.handle_start(&msg, ns),
            &Payload::Go => self.handle_go(&msg, ns),
            &Payload::BackChild => self.handle_back_child(&msg, ns),
            &Payload::BackNotAChild => self.handle_back_not_a_child(&msg, ns),
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
