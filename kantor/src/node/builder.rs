//! Implementation of the builder pattern for the `Node' structure.

use crate::{graph::GraphMsg, protocol::ProtocolMsg, ActorId};
use actix::{dev::ToEnvelope, prelude::*};
use std::marker::PhantomData;

use super::Node;

mod states {
    pub struct WithActorId {}
    pub struct Ready {}
}

/// Implements the Builder pattern for the `Node`.
/// The builder can initialized either from an `ActorId`.
/// The next step is to set the address, while the final
/// step is to build the `Node` instance by calling `build` function.
pub struct NodeBuilder<A, S = states::WithActorId>
where
    A: Actor,
{
    aid: ActorId,
    addr: Option<Addr<A>>,
    phantom: PhantomData<S>,
}

impl<A> From<ActorId> for NodeBuilder<A>
where
    A: Actor,
{
    fn from(value: ActorId) -> Self {
        Self::from_aid(value)
    }
}

impl<A> NodeBuilder<A>
where
    A: Actor,
{
    /// Initializes the building chain by creating a new
    /// instance of `Builder` from an `ActorId`.
    pub fn from_aid(aid: ActorId) -> Self {
        Self {
            aid,
            addr: None,
            phantom: PhantomData,
        }
    }
}

impl<A> NodeBuilder<A>
where
    A: Actor,
{
    /// Continues the building chain by setting the address for
    /// the new `Node` instance. Once this is calles, you can
    /// build the new `Node` instance.
    pub fn with_addr(self, addr: Addr<A>) -> NodeBuilder<A, states::Ready> {
        NodeBuilder::<A, states::Ready> {
            aid: self.aid,
            addr: Some(addr),
            phantom: PhantomData,
        }
    }
}

impl<A> NodeBuilder<A, states::Ready>
where
    A: Actor,
{
    /// Finalizes the building chain by building the new `Node` instance.
    pub fn build<P>(self) -> Node<A, P>
    where
        P: Send + 'static,
        A: Handler<ProtocolMsg<P>>,
        A::Context: ToEnvelope<A, ProtocolMsg<P>>,
        A: Handler<GraphMsg<ProtocolMsg<P>>>,
        A::Context: ToEnvelope<A, GraphMsg<ProtocolMsg<P>>>,
    {
        Node::new(self.aid, self.addr.unwrap())
    }
}
