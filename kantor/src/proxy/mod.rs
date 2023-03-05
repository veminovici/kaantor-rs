//! A implementation for a proxy for a remote node.

pub mod builder;
mod metrics;
mod mid;

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
    aid: ActorId,
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
    fn debug_op(&self, op: &str, mid: &MessageId, sid: &ActorId) {
        debug!("{}'ng {} {}-->{}", op, mid, sid, &self.aid)
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
    pub async fn send(&mut self, sid: &ActorId, msg: M) -> Result<M::Result, MailboxError> {
        let mid = self.incrememt_mid();
        self.debug_op("send", &mid, sid);

        match self.recipient.send(msg).await {
            Ok(x) => {
                self.metrics.record_success();
                Ok(x)
            }
            Err(e) => {
                error!("send'fd mid={:?}", mid);
                self.metrics.record_failure();
                Err(e)
            }
        }
    }

    /// Tries to send a message `M` to the remote node.
    pub fn try_send(&mut self, sid: &ActorId, msg: M) -> Result<(), SendError<M>> {
        let mid = self.incrememt_mid();
        self.debug_op("try_send", &mid, sid);

        match self.recipient.try_send(msg) {
            Ok(x) => {
                self.metrics.record_success();
                Ok(x)
            }
            Err(e) => {
                error!("send'fd mid={:?}", mid);
                self.metrics.record_failure();
                Err(e)
            }
        }
    }

    /// Does send a message to the remote node.
    pub fn do_send(&mut self, sid: &ActorId, msg: M) {
        let mid = self.incrememt_mid();
        self.debug_op("do_send", &mid, sid);

        self.recipient.do_send(msg);
        self.metrics.record_success();
    }
}
