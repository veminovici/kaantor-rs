//! A module which implements the main functionality
//! for the `Node`, the key actor for the distributed systems.

pub mod builder;
mod proxies;

use crate::{
    graph::CfgMessage,
    protocol::ProtocolMsg,
    proxy::{builder::Builder, Proxy},
    ActorId,
};
use actix::{dev::ToEnvelope, prelude::*};
pub use proxies::*;

/// The actor node for the distributed systems.
/// The actor can receive `CfgMessage` and protocol messages with a
/// defined `P` payload.
pub struct Node<A, P>
where
    P: Send,
    A: Actor,
    A: Handler<ProtocolMsg<P>>,
    A::Context: ToEnvelope<A, ProtocolMsg<P>>,
    A: Handler<CfgMessage<ProtocolMsg<P>>>,
    A::Context: ToEnvelope<A, CfgMessage<ProtocolMsg<P>>>,
{
    aid: ActorId,
    addr: Addr<A>,
    cfg: Proxy<CfgMessage<ProtocolMsg<P>>>,
}

impl<A, P> Node<A, P>
where
    P: Send + 'static,
    A: Actor,
    A: Handler<ProtocolMsg<P>>,
    A::Context: ToEnvelope<A, ProtocolMsg<P>>,
    A: Handler<CfgMessage<ProtocolMsg<P>>>,
    A::Context: ToEnvelope<A, CfgMessage<ProtocolMsg<P>>>,
{
    fn new(aid: ActorId, addr: Addr<A>) -> Self {
        let cfg = Self::get_cfg_proxy(aid, addr.clone());
        Self { aid, addr, cfg }
    }

    fn get_cfg_proxy(aid: ActorId, addr: Addr<A>) -> Proxy<CfgMessage<ProtocolMsg<P>>> {
        let recipient = addr.recipient::<CfgMessage<ProtocolMsg<P>>>();
        Builder::from_aid(aid).with_recipient(recipient).build()
    }

    /// Creates a proxy for protocol messages. This proxy will be stored
    /// the the neighbour nodes, so they can communicate with the current node.
    pub fn as_proxy(&self) -> Proxy<ProtocolMsg<P>> {
        let recipient = self.addr.clone().recipient::<ProtocolMsg<P>>();
        Builder::from_aid(self.aid)
            .with_recipient(recipient)
            .build()
    }

    /// Sends to the current node a configuration message to add a new neighbour proxy
    /// to the current node.
    pub async fn add_proxy(&mut self, proxy: Proxy<ProtocolMsg<P>>) -> Result<(), MailboxError> {
        let msg = CfgMessage::AddProxy(proxy);
        self.cfg.send(&self.aid, msg).await
    }

    /// Send a protocol message to the node.
    pub async fn send(
        &mut self,
        msg: ProtocolMsg<P>,
    ) -> Result<<ProtocolMsg<P> as Message>::Result, MailboxError> {
        self.addr.send(msg).await
    }

    /// Try to send a protocol message to the ndoe.
    pub fn try_send(&mut self, msg: ProtocolMsg<P>) -> Result<(), SendError<ProtocolMsg<P>>> {
        self.addr.try_send(msg)
    }

    /// Does send a protocol message to the node.
    pub fn do_send(&mut self, msg: ProtocolMsg<P>) {
        self.addr.do_send(msg)
    }
}
