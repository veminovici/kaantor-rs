//! The builder pattern implementation for the `Proxy`.
//!
use std::marker::PhantomData;

use actix::Recipient;

mod states {
    pub struct WithActorId {}
    pub struct Ready {}
}

use crate::ActorId;

use super::Proxy;

/// The builder for the `Proxy`
pub struct ProxyBuilder<M, S = states::WithActorId>
where
    M: actix::Message + Send,
    M::Result: Send,
{
    aid: ActorId,
    recipient: Option<Recipient<M>>,
    phantom: PhantomData<S>,
}

impl<M> From<ActorId> for ProxyBuilder<M>
where
    M: actix::Message + Send,
    M::Result: Send,
{
    fn from(value: ActorId) -> Self {
        Self::from_aid(value)
    }
}

impl<M> ProxyBuilder<M>
where
    M: actix::Message + Send,
    M::Result: Send,
{
    /// Initializes the builder chain from an `ActorId` value.
    pub fn from_aid(aid: ActorId) -> Self {
        Self {
            aid,
            recipient: None,
            phantom: PhantomData,
        }
    }

    /// Continues the building chain by setting the recipient.
    pub fn with_recipient(self, recipient: Recipient<M>) -> ProxyBuilder<M, states::Ready> {
        ProxyBuilder::<M, states::Ready> {
            aid: self.aid,
            recipient: Some(recipient),
            phantom: PhantomData,
        }
    }
}

impl<M> ProxyBuilder<M, states::Ready>
where
    M: actix::Message + Send,
    M::Result: Send,
{
    /// Finalizes the buidling chain by building a new `Proxy` instance.
    pub fn build(self) -> Proxy<M> {
        Proxy::new(self.aid, self.recipient.unwrap())
    }
}
