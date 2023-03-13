use actix::prelude::*;
use kaantor::{
    protocol::{Builder, Session},
    NodeActor, *,
};
use log::{debug, info};
use std::fmt::Debug;

use ptree::*;

#[derive(Clone)]
enum Payload {
    Start,
    Go,
    BackYes,
    BackNotAChild,
}

impl Debug for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "START"),
            Self::Go => write!(f, "GO"),
            Self::BackYes => write!(f, "BACK CHILD"),
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

struct Handler {
    aid: ActorId,
    parent: Parent,
    children: Vec<ActorId>,
    visited: Vec<ActorId>,
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

    fn handle_start(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.parent = Parent::Root;
        let session = msg.session().clone();

        if ns.count() == 0 {
            // we are done
            return ContinuationHandler::Done
        } else {
            self.send_go_to_all_except(session, vec![])
        }
    }

    fn handle_node_done(&mut self) -> ContinuationHandler<Payload> {
        self.debug_spanning_node();
        ContinuationHandler::Done
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
                if ns.eq(self.visited.clone()) {
                    self.handle_node_done()
                } else {
                    // pick one of the neighbours that is not visited
                    // and send a GO message to it
                    todo!()
                }
            },
            Parent::Root => {
                // send a BACK(no) to the sender.
                todo!()
            },
            Parent::Parent(_) => {
                // send a BACK(no) to the sender.
                todo!()
            },
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

    fn handle_back(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.visited.push(msg.sender().as_aid());

        if ns.eq(self.visited.clone()) {
            self.handle_node_done()
        } else {
            // pick one of the neighbours that is not visited
            // and send a GO message to it
            todo!()
        }
}

    fn send_go_to_all_except(
        &self,
        session: Session,
        except: Vec<ActorId>,
    ) -> ContinuationHandler<Payload> {
        let msg = Builder::with_from_actor(self.aid)
            .with_to_all_actors()
            .with_session(session)
            .with_payload(Payload::Go)
            .with_sender(self.aid)
            .build();

        ContinuationHandler::SendToAllNodesExcept(msg, except)
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
            &Payload::BackYes => self.handle_back_child(&msg, ns),
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
