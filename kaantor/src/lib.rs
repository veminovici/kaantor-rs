#![warn(missing_docs)]

//! A crate for distributed systems

mod actor;
pub mod graph;
pub mod node;
pub mod protocol;
mod proxy;

pub use actor::*;
pub use node::{Node, Proxies};

use actix::{dev::ToEnvelope, prelude::*};
use log::debug;
use protocol::Message as PMsg;
use std::fmt::Debug;

type GMsg<P> = graph::GraphMsg<PMsg<P>>;

/// The follow up
pub enum ContinuationHandler<P> {
    /// Send a messagge to a neighbour
    SendToNode(ActorId, protocol::Message<P>),
    /// Send a message to all neighbours
    SendToAllNodes(protocol::Message<P>),
    /// Send a message to all neightbours excepts few
    SendToAllNodesExcept(protocol::Message<P>, Vec<ActorId>),
    /// We are done
    Done,
}

/// The trait which defines the behaviour of a node.
pub trait ProtocolHandler {
    /// The type of payload for the messages.
    type Payload: Send;

    /// Returns the `ActorId` for the current handler.
    fn aid(&self) -> ActorId;

    /// Processes the received message.
    fn receive(
        &mut self,
        neighbours: impl Iterator::<Item = ActorId>, // &Proxies<Self::Payload>,
        msg: protocol::Message<Self::Payload>,
    ) -> ContinuationHandler<Self::Payload>;
}

/// Convenience type
pub type NodeHandler<T> = Node<NodeActor<T>, <T as ProtocolHandler>::Payload>;

/// Add a bi-directional connection between two ndoes.
pub async fn add_edge<A, P>(a: &mut Node<A, P>, b: &mut Node<A, P>)
where
    P: Send + 'static,
    A: Actor,
    A: Handler<PMsg<P>>,
    A::Context: ToEnvelope<A, PMsg<P>>,
    A: Handler<GMsg<P>>,
    A::Context: ToEnvelope<A, GMsg<P>>,
{
    debug!("add edge [{:?}-{:?}]", a.aid(), b.aid());

    let pxy_a = a.as_proxy();
    let pxy_b = b.as_proxy();

    let _ = a.add_proxy(pxy_b).await;
    let _ = b.add_proxy(pxy_a).await;
}

/// Returns the debuging version of an iterator
pub fn debug_iter<'a, I>(ns: impl Iterator<Item = &'a I>) -> String
where
    I: Debug + 'a,
{
    ns.fold("".to_string(), |acc, n| {
        if acc.is_empty() {
            format!("{n:?}")
        } else {
            format!("{acc}, {n:?}")
        }
    })
}
