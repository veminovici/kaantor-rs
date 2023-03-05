pub mod builder;
mod cfgmsg;
mod proxies;

use crate::{
    message::Message as Msg,
    proxy::{builder::Builder, Proxy},
    ActorId,
};
use actix::{dev::ToEnvelope, prelude::*};
pub use cfgmsg::*;
pub use proxies::*;

pub struct Node<A, P>
where
    P: Send,
    A: Actor,
    A: Handler<Msg<P>>,
    A::Context: ToEnvelope<A, Msg<P>>,
    A: Handler<CfgMessage<Msg<P>>>,
    A::Context: ToEnvelope<A, CfgMessage<Msg<P>>>,
{
    aid: ActorId,
    addr: Addr<A>,
    cfg: Proxy<CfgMessage<Msg<P>>>,
}

impl<A, P> Node<A, P>
where
    P: Send + 'static,
    A: Actor,
    A: Handler<Msg<P>>,
    A::Context: ToEnvelope<A, Msg<P>>,
    A: Handler<CfgMessage<Msg<P>>>,
    A::Context: ToEnvelope<A, CfgMessage<Msg<P>>>,
{
    pub fn new(aid: ActorId, addr: Addr<A>) -> Self {
        let cfg = Self::get_cfg_proxy(aid, addr.clone());
        Self { aid, addr, cfg }
    }

    fn get_cfg_proxy(aid: ActorId, addr: Addr<A>) -> Proxy<CfgMessage<Msg<P>>> {
        let recipient = addr.recipient::<CfgMessage<Msg<P>>>();
        Builder::from_aid(aid).with_recipient(recipient).build()
    }

    pub fn as_proxy(&self) -> Proxy<Msg<P>> {
        let recipient = self.addr.clone().recipient::<Msg<P>>();
        Builder::from_aid(self.aid)
            .with_recipient(recipient)
            .build()
    }

    pub async fn add_proxy(&mut self, proxy: Proxy<Msg<P>>) -> Result<(), MailboxError> {
        let msg = CfgMessage::AddProxy(proxy);
        self.cfg.send(&self.aid, msg).await
    }

    pub async fn send(&mut self, msg: Msg<P>) -> Result<<Msg<P> as Message>::Result, MailboxError> {
        self.addr.send(msg).await
    }

    pub fn try_send(&mut self, msg: Msg<P>) -> Result<(), SendError<Msg<P>>> {
        self.addr.try_send(msg)
    }

    pub fn do_send(&mut self, msg: Msg<P>) {
        self.addr.do_send(msg)
    }
}
