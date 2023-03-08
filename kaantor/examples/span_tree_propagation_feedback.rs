use actix::prelude::*;
use kaantor::{
    protocol::{Builder, Session},
    NodeActor, *,
};
use log::{debug, info};
use std::fmt::Debug;

use ptree::*;

#[derive(Clone)]
struct STNode {
    root: ActorId,
    children: Vec<ActorId>,
}

impl Debug for STNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}->[{}]", self.root, debug_iter(self.children.iter()))
    }
}

#[derive(Clone)]
enum Payload {
    Start,
    Go,
    BackChild(Vec<STNode>),
    BackNoChild,
}

impl Debug for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "START"),
            Self::Go => write!(f, "GO"),
            Self::BackChild(ns) => {
                write!(f, "BACK CHILD | {}", debug_iter(ns.iter()))
            }
            Self::BackNoChild => write!(f, "BACK NOT A CHILD"),
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
    exp_messages: usize,
    nodes: Vec<STNode>,
}

impl Handler {
    /// Builds a handler
    fn build(aid: ActorId) -> Node<NodeActor<Self>, Payload> {
        NodeActor::build(Self {
            aid,
            parent: Parent::NoParent,
            children: vec![],
            exp_messages: 0,
            nodes: vec![],
        })
    }

    #[inline]
    fn debug_spanning_node(&self) {
        info!(
            "SPANNING TREE NODE: {:?} p={:?} cs={:?}",
            self.aid, self.parent, self.children
        );
    }

    fn build_child<'a>(&'a self, tb: &'a mut TreeBuilder, nid: ActorId) -> &'a mut TreeBuilder {
        self.nodes
            .iter()
            .find(|n| n.root == nid)
            .map(|n| {
                if n.children.len() == 0 {
                    tb.add_empty_child(format!("{:?}", nid))
                } else {
                    let tb = tb.begin_child(format!("{:?}", nid));
                    let tb = n
                        .children
                        .iter()
                        .fold(tb, |tb, cid| self.build_child(tb, *cid));
                    // tb.add_empty_child(format!("{}", nid))
                    tb.end_child()
                }
            })
            .unwrap()
    }

    fn debug_spanning_tree(&self) {
        let mut tb = TreeBuilder::new("SPANNING-TREE".to_string());
        let tb = self.build_child(&mut tb, self.aid).build();
        let _ = print_tree(&tb).unwrap();
    }

    /// The node received a START. It marks itself as ROOT, sends a GO message to all its
    /// neighbours while expecting back a number of messages equal with the number of neighbours.
    fn handle_start(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.parent = Parent::Root;
        self.exp_messages = ns.count();
        let session = msg.session().clone();

        self.send_go_to_all_except(session, vec![])
    }

    fn handle_go(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        let session = msg.session();

        match self.parent {
            Parent::NoParent => {
                let sender = msg.sender().as_aid();
                self.parent = Parent::Parent(sender);
                self.exp_messages = ns.count() - 1;

                if self.exp_messages != 0 {
                    // Continue the discovery of the spanning tree by sending
                    // a GO message to all the neighbours except the one that
                    // sent us the GO.
                    self.send_go_to_all_except(session.clone(), vec![sender])
                } else {
                    // Finalize the spanning tree search for this node.
                    // Send back_child to the node that sent us the GO meesage.
                    self.debug_spanning_node();
                    self.send_back_child_to_node(sender, *session)
                }
            }
            Parent::Root => {
                let sender = msg.sender().as_aid();
                self.send_back_no_child_to_node(*session, sender)
            }
            Parent::Parent(_) => {
                let sender = msg.sender().as_aid();
                self.send_back_no_child_to_node(*session, sender)
            }
        }
    }

    #[inline]
    fn handle_back_no_child(
        &mut self,
        msg: &protocol::Message<Payload>,
        proxies: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.handle_back(msg, proxies)
    }

    #[inline]
    fn handle_back_child(
        &mut self,
        msg: &protocol::Message<Payload>,
        ns: Vec<STNode>,
        proxies: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        let sender = msg.sender().as_aid();
        self.children.push(sender);
        self.nodes.extend(ns);

        self.handle_back(msg, proxies)
    }

    fn handle_back(
        &mut self,
        msg: &protocol::Message<Payload>,
        _proxies: impl Iterator<Item = ActorId>,
    ) -> ContinuationHandler<Payload> {
        self.exp_messages -= 1;
        let session = msg.session();

        // info!("{} em={} p={:?}", self.aid, self.exp_messages, self.parent);

        if self.exp_messages == 0 {
            self.debug_spanning_node();

            match self.parent {
                Parent::NoParent => panic!("We shoud not be here"),
                Parent::Root => {
                    let node = STNode {
                        root: self.aid,
                        children: self.children.clone(),
                    };

                    self.nodes.extend(vec![node]);

                    info!("Finished the spanning tree");
                    self.debug_spanning_tree();

                    ContinuationHandler::Done
                }
                Parent::Parent(pid) => self.send_back_child_to_node(pid, *session),
            }
        } else {
            ContinuationHandler::Done
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

    fn send_back_no_child_to_node(
        &self,
        session: Session,
        sender: ActorId,
    ) -> ContinuationHandler<Payload> {
        let msg = Builder::with_from_actor(self.aid)
            .with_to_actor(sender)
            .with_session(session)
            .with_payload(Payload::BackNoChild)
            .with_sender(self.aid)
            .build();

        ContinuationHandler::SendToNode(sender, msg)
    }

    fn send_back_child_to_node(
        &self,
        pid: ActorId,
        session: Session,
    ) -> ContinuationHandler<Payload> {
        // info!("Back_Child (0) to_actor={hid} send_to_node={hid}");
        let node = STNode {
            root: self.aid,
            children: self.children.clone(),
        };

        let mut ns = self.nodes.clone();
        ns.extend(vec![node]);

        let msg = Builder::with_from_actor(self.aid)
            .with_to_actor(pid)
            .with_session(session)
            .with_payload(Payload::BackChild(ns))
            .with_sender(self.aid)
            .build();

        ContinuationHandler::SendToNode(pid, msg)
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
        match &msg.payload() {
            &Payload::Start => self.handle_start(&msg, proxies.aids()),
            &Payload::Go => self.handle_go(&msg, proxies.aids()),
            &Payload::BackNoChild => self.handle_back_no_child(&msg, proxies.aids()),
            &Payload::BackChild(ns) => self.handle_back_child(&msg, ns.clone(), proxies.aids()),
        }
    }
}

fn main() {
    env_logger::init();
    debug!("Starting the example SPANNING TREE");

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
