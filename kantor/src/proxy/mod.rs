mod metrics;
mod mid;

use crate::message::Message as Msg;
use actix::prelude::*;
use log::{debug, error};
use self::{mid::MessageId, metrics::Metrics};

pub struct Proxy<P>
where
    P: Send,
{
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

    pub(crate) fn new(recipient: Recipient<Msg<P>>) -> Self {
        Self {
            mid: Default::default(),
            metrics: Default::default(),
            recipient,
        }
    }

    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }

    pub async fn send(&mut self, msg: Msg<P>) -> Result<<Msg<P> as Message>::Result, MailboxError> {
        let mid = self.incrememt_mid();
        debug!("send'ng mid={:?} msg={:?}", mid, msg);

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
        debug!("try_send'ng mid={:?} msg={:?}", mid, msg);

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
        debug!("do_send'ng mid={:?} msg={:?}", mid, msg);

        self.recipient.do_send(msg);
        self.metrics.record_success();
    }
}
