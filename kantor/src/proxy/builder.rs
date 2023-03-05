use std::marker::PhantomData;

use actix::Recipient;

mod states {
    pub struct WithActorId {}
    pub struct Ready {}
}

use crate::message::{ActorId, Message as Msg};

use super::Proxy;

pub struct Builder<P, S = states::WithActorId>
where
    P: Send,
{
    aid: ActorId,
    recipient: Option<Recipient<Msg<P>>>,
    phantom: PhantomData<S>,
}

impl<P> Builder<P>
where
    P: Send,
{
    pub fn from_aid(aid: ActorId) -> Self {
        Self {
            aid,
            recipient: None,
            phantom: PhantomData,
        }
    }

    pub fn with_recipient(self, recipient: Recipient<Msg<P>>) -> Builder<P, states::Ready> {
        Builder::<P, states::Ready> {
            aid: self.aid,
            recipient: Some(recipient),
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::Ready>
where
    P: Send,
{
    pub fn build(self) -> Proxy<P> {
        Proxy::new(self.aid, self.recipient.unwrap())
    }
}
