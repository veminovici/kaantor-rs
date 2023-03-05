//! A module which implements the main functionality
//! for the `Node`, the key actor for the distributed systems.

pub mod builder;
mod proxies;

//pub use builder::*;
pub use proxies::*;

use crate::{
    graph::GraphMsg,
    protocol::{states, Builder as ProBuilder, ProtocolMsg},
    proxy::{Builder as PxyBuilder, Proxy},
    ActorId,
};
use actix::{dev::ToEnvelope, prelude::*};

type PMsg<P> = ProtocolMsg<P>;
type GMsg<P> = GraphMsg<ProtocolMsg<P>>;

/// The actor node for the distributed systems.
/// The actor can receive `CfgMessage` and protocol messages with a
/// defined `P` payload.
pub struct Node<A, P>
where
    P: Send,
    A: Actor,
    A: Handler<PMsg<P>>,
    A::Context: ToEnvelope<A, PMsg<P>>,
    A: Handler<GMsg<P>>,
    A::Context: ToEnvelope<A, GMsg<P>>,
{
    aid: ActorId,
    addr: Addr<A>,
    cfg: Proxy<GMsg<P>>,
}

impl<A, P> Node<A, P>
where
    P: Send + 'static,
    A: Actor,
    A: Handler<PMsg<P>>,
    A::Context: ToEnvelope<A, PMsg<P>>,
    A: Handler<GMsg<P>>,
    A::Context: ToEnvelope<A, GMsg<P>>,
{
    fn new(aid: ActorId, addr: Addr<A>) -> Self {
        let cfg = Self::get_cfg_proxy(aid, addr.clone());
        Self { aid, addr, cfg }
    }

    /// Gets the acotr identifier.
    pub fn aid(&self) -> &ActorId {
        &self.aid
    }

    /// Returns a protocol builder.
    pub fn protocol_builder(&self) -> ProBuilder<P, states::WithFromId> {
        ProBuilder::from_aid(self.aid)
    }

    fn get_cfg_proxy(aid: ActorId, addr: Addr<A>) -> Proxy<GMsg<P>> {
        let recipient = addr.recipient::<GMsg<P>>();
        PxyBuilder::from_aid(aid).with_recipient(recipient).build()
    }

    /// Creates a proxy for protocol messages. This proxy will be stored
    /// the the neighbour nodes, so they can communicate with the current node.
    pub fn as_proxy(&self) -> Proxy<PMsg<P>> {
        let recipient = self.addr.clone().recipient::<PMsg<P>>();
        PxyBuilder::from_aid(self.aid)
            .with_recipient(recipient)
            .build()
    }

    /// Sends to the current node a configuration message to add a new neighbour proxy
    /// to the current node.
    pub async fn add_proxy(&mut self, proxy: Proxy<PMsg<P>>) -> Result<(), MailboxError> {
        let msg = GraphMsg::AddProxy(proxy);
        self.cfg.send(&self.aid, msg).await
    }

    /// Send a protocol message to the node.
    pub async fn send(
        &mut self,
        msg: PMsg<P>,
    ) -> Result<<PMsg<P> as Message>::Result, MailboxError> {
        self.addr.send(msg).await
    }

    /// Try to send a protocol message to the ndoe.
    pub fn try_send(&mut self, msg: PMsg<P>) -> Result<(), SendError<PMsg<P>>> {
        self.addr.try_send(msg)
    }

    /// Does send a protocol message to the node.
    pub fn do_send(&mut self, msg: ProtocolMsg<P>) {
        self.addr.do_send(msg)
    }
}
