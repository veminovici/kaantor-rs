pub mod builder;
mod metrics;
mod mid;

use self::{metrics::Metrics, mid::MessageId};
use crate::message::{ActorId, Message as Msg};
use actix::prelude::*;
use log::{debug, error};

#[derive(Debug)]
pub struct Proxy<P>
where
    P: Send,
{
    aid: ActorId,
    mid: MessageId,
    metrics: Metrics,
    recipient: Recipient<Msg<P>>,
}

impl<P> Proxy<P>
where
    P: Send,
{
    #[inline]
    fn incrememt_mid(&mut self) -> MessageId {
        self.mid.increment_mid()
    }

    #[inline]
    fn debug_op(&self, op: &str, mid: &MessageId, msg: &Msg<P>) {
        debug!(
            "{}'ng {} mh=[{}-->{}] ft=[{}>>>{}]",
            op,
            mid,
            msg.hid(),
            &self.aid,
            msg.fid(),
            msg.tid()
        )
    }

    fn new(aid: ActorId, recipient: Recipient<Msg<P>>) -> Self {
        Self {
            aid,
            mid: Default::default(),
            metrics: Default::default(),
            recipient,
        }
    }

    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }

    pub fn aid(&self) -> &ActorId {
        &self.aid
    }

    pub async fn send(&mut self, msg: Msg<P>) -> Result<<Msg<P> as Message>::Result, MailboxError> {
        let mid = self.incrememt_mid();
        self.debug_op("send", &mid, &msg);

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

    pub fn try_send(&mut self, msg: Msg<P>) -> Result<(), SendError<Msg<P>>> {
        let mid = self.incrememt_mid();
        self.debug_op("try_send", &mid, &msg);

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

    pub fn do_send(&mut self, msg: Msg<P>) {
        let mid = self.incrememt_mid();
        self.debug_op("do_send", &mid, &msg);

        self.recipient.do_send(msg);
        self.metrics.record_success();
    }
}
