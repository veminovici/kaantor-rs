use actix::prelude::*;
use kantor::{
    protocol::{Builder, SessionId},
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
    exp_messages: usize,
}

impl Handler {
    /// Builds a handler
    fn build(aid: ActorId) -> Node<NodeActor<Self>, Payload> {
        NodeActor::build(Self {
            aid,
            parent: Parent::NoParent,
            children: vec![],
            exp_messages: 0,
        })
    }

    #[inline]
    fn debug_spanning_tree(&self) {
        info!(
            "SPANNING TREE NODE: {} p={:?} cs={:?}",
            self.aid, self.parent, self.children
        );
    }

    /// The node received a START. It marks itself as ROOT, sends a GO message to all its
    /// neighbours while expecting back a number of messages equal with the number of neighbours.
    fn handle_start(
        &mut self,
        msg: protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.parent = Parent::Root;
        self.exp_messages = ns.count();
        let sid = msg.sid().clone();

        // info!("{} em={} p={:?}", self.aid, self.exp_messages, self.parent);
        self.send_go_to_all_except(sid, vec![])
    }

    fn handle_go(
        &mut self,
        msg: protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        let sid = msg.sid();

        match self.parent {
            Parent::NoParent => {
                self.parent = Parent::Parent(msg.hid().aid());
                self.exp_messages = ns.count() - 1;
                let hid = msg.hid().aid();

                // info!("{} em={} p={:?}", self.aid, self.exp_messages, self.parent);

                if self.exp_messages != 0 {
                    // Continue the discovery of the spanning tree by sending
                    // a GO message to all the neighbours except the one that
                    // sent us the GO.
                    self.send_go_to_all_except(sid.clone(), vec![hid])
                } else {
                    // Finalize the spanning tree search for this node.
                    // Send back_child to the node that sent us the GO meesage.
                    self.debug_spanning_tree();
                    self.send_back_child_to_node(*sid, hid)
                }
            }
            Parent::Root => {
                let hid = msg.hid().aid();
                self.send_back_no_child_to_node(*sid, hid)
            }
            Parent::Parent(_) => {
                let hid = msg.hid().aid();
                self.send_back_no_child_to_node(*sid, hid)
            }
        }
    }

    #[inline]
    fn handle_back_no_child(
        &mut self,
        msg: protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.handle_back(msg, ns)
    }

    #[inline]
    fn handle_back_child(
        &mut self,
        msg: protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.children.push(msg.hid().aid()); // diff
        self.handle_back(msg, ns)
    }

    fn handle_back(
        &mut self,
        msg: protocol::Message<Payload>,
        _ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.exp_messages -= 1;
        let sid = msg.sid();

        // info!("{} em={} p={:?}", self.aid, self.exp_messages, self.parent);

        if self.exp_messages == 0 {
            self.debug_spanning_tree();

            match self.parent {
                Parent::NoParent => panic!("We shoud not be here"),
                Parent::Root => {
                    info!("Finished the spanning tree");
                    ContinuationHandler::Done
                }
                Parent::Parent(pid) => {
                    // Finish the spanning tree discovery for this node.
                    // Send back a back_child to the parent node.
                    // info!("Back_Child to_actor={pid} send_to_node={pid}");
                    let msg = Builder::with_from_actor(self.aid)
                        .with_to_actor(pid)
                        .with_session(sid.clone())
                        .with_payload(Payload::BackChild)
                        .with_hid(self.aid)
                        .build();

                    ContinuationHandler::SendToNode(pid, msg)
                }
            }
        } else {
            ContinuationHandler::Done
        }
    }

    fn send_go_to_all_except(
        &self,
        sid: SessionId,
        except: Vec<ActorId>,
    ) -> ContinuationHandler<Payload> {
        let msg = Builder::with_from_actor(self.aid)
            .with_to_all_actors()
            .with_session(sid)
            .with_payload(Payload::Go)
            .with_hid(self.aid)
            .build();

        ContinuationHandler::SendToAllNodesExcept(msg, except)
    }

    fn send_back_no_child_to_node(&self, sid: SessionId, hid: ActorId) -> ContinuationHandler<Payload> {
        // info!("{} em={} p={:?}", self.aid, self.exp_messages, self.parent);

        let msg = Builder::with_from_actor(self.aid)
        .with_to_actor(hid)
        .with_session(sid)
        .with_payload(Payload::BackNoChild)
        .with_hid(self.aid)
        .build();

        ContinuationHandler::SendToNode(hid, msg)
    }

    fn send_back_child_to_node(&self, sid: SessionId, hid: ActorId) -> ContinuationHandler<Payload> {
        // info!("Back_Child (0) to_actor={hid} send_to_node={hid}");

        let msg = Builder::with_from_actor(self.aid)
            .with_to_actor(hid)
            .with_session(sid)
            .with_payload(Payload::BackChild)
            .with_hid(self.aid)
            .build();

        ContinuationHandler::SendToNode(hid, msg)
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
            &Payload::Start => self.handle_start(msg, proxies.aids()),
            &Payload::Go => self.handle_go(msg, proxies.aids()),
            &Payload::BackNoChild => self.handle_back_no_child(msg, proxies.aids()),
            &Payload::BackChild => self.handle_back_child(msg, proxies.aids()),
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
            .with_to_actor(*p1.aid())
            .with_session(50.into())
            .with_payload(Payload::Start)
            .with_hid(*p1.aid())
            .build();

        let _ = p1.send(msg).await;
    });

    println!("Finished the test");
}
