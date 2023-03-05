//! Implementation of the builder pattern for the `Node' structure.

use crate::{message::Message as Msg, ActorId};
use actix::{dev::ToEnvelope, prelude::*};
use std::marker::PhantomData;

use super::{CfgMessage, Node};

mod states {
    pub struct WithActorId {}
    pub struct Ready {}
}

/// Implements the Builder pattern for the `Node`.
/// The builder can initialized either from an `ActorId`.
/// The next step is to set the address, while the final
/// step is to build the `Node` instance by calling `build` function.
pub struct Builder<A, S = states::WithActorId>
where
    A: Actor,
{
    aid: ActorId,
    addr: Option<Addr<A>>,
    phantom: PhantomData<S>,
}

impl<A> From<ActorId> for Builder<A>
where
    A: Actor,
{
    fn from(value: ActorId) -> Self {
        Self::from_aid(value)
    }
}

impl<A> Builder<A>
where
    A: Actor
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

impl<A> Builder<A>
where
    A: Actor,
{
    /// Continues the building chain by setting the address for
    /// the new `Node` instance. Once this is calles, you can
    /// build the new `Node` instance.
    pub fn with_addr(self, addr: Addr<A>) -> Builder<A, states::Ready> {
        Builder::<A, states::Ready> {
            aid: self.aid,
            addr: Some(addr),
            phantom: PhantomData,
        }
    }
}

impl<A> Builder<A, states::Ready>
where
    A: Actor,
{
    /// Finalizes the building chain by building the new `Node` instance.
    pub fn build<P>(self) -> Node<A, P>
    where
        P: Send + 'static,
        A: Handler<Msg<P>>,
        A::Context: ToEnvelope<A, Msg<P>>,
        A: Handler<CfgMessage<Msg<P>>>,
        A::Context: ToEnvelope<A, CfgMessage<Msg<P>>>,
    {
        Node::new(self.aid, self.addr.unwrap())
    }
}
