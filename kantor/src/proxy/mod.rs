//! A implementation for a proxy for a remote node.

mod builder;
mod metrics;
mod mid;

pub use builder::*;

use crate::ActorId;

use self::{metrics::Metrics, mid::MessageId};
use actix::prelude::*;
use log::{debug, error};

/// Represents a proxy which can sends a `M` message.
#[derive(Debug)]
pub struct Proxy<M>
where
    M: Message + Send,
    M::Result: Send,
{
    pub(crate) aid: ActorId,
    mid: MessageId,
    metrics: Metrics,
    recipient: Recipient<M>,
}

impl<M> Proxy<M>
where
    M: Message + Send,
    M::Result: Send,
{
    #[inline]
    fn incrememt_mid(&mut self) -> MessageId {
        self.mid.increment_mid()
    }

    #[inline]
    fn debug_op(&self, op: &str, from: &ActorId) {
        let to = &self.aid;
        debug!("{}'ng [{}-->{}]", op, from, to)
    }

    fn new(aid: ActorId, recipient: Recipient<M>) -> Self {
        Self {
            aid,
            mid: Default::default(),
            metrics: Default::default(),
            recipient,
        }
    }

    /// Gets the metrics for the current proxy instance.
    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }

    /// Gets the actor identifier.
    pub fn aid(&self) -> &ActorId {
        &self.aid
    }

    /// Sends a message `M` to the remote node.
    pub async fn send(&mut self, from: &ActorId, msg: M) -> Result<M::Result, MailboxError> {
        let mid = self.incrememt_mid();
        self.debug_op("send", from);

        match self.recipient.send(msg).await {
            Ok(x) => {
                self.metrics.record_success();
                Ok(x)
            }
            Err(e) => {
                error!("send'fd [{}]", mid);
                self.metrics.record_failure();
                Err(e)
            }
        }
    }

    /// Tries to send a message `M` to the remote node.
    pub fn try_send(&mut self, from: &ActorId, msg: M) -> Result<(), SendError<M>> {
        let mid = self.incrememt_mid();
        self.debug_op("try_send", from);

        match self.recipient.try_send(msg) {
            Ok(x) => {
                self.metrics.record_success();
                Ok(x)
            }
            Err(e) => {
                error!("send'fd [{}]", mid);
                self.metrics.record_failure();
                Err(e)
            }
        }
    }

    /// Does send a message to the remote node.
    pub fn do_send(&mut self, from: &ActorId, msg: M) {
        let _mid = self.incrememt_mid();
        self.debug_op("do_send", from);

        self.recipient.do_send(msg);
        self.metrics.record_success();
    }
}
