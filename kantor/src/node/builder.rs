use crate::{message::Message as Msg, ActorId};
use actix::{dev::ToEnvelope, prelude::*};
use std::marker::PhantomData;

use super::{CfgMessage, Node};

mod states {
    pub struct WithActorId {}
    pub struct Ready {}
}

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
        Self {
            aid: value,
            addr: None,
            phantom: PhantomData,
        }
    }
}

impl<A> Builder<A>
where
    A: Actor,
{
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
