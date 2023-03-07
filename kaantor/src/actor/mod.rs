//! The actor functionality

mod aid;

pub use aid::*;

use std::fmt::Debug;

use crate::{node::Builder as NBuilder, protocol::Message as ProMsg, *};
use actix::prelude::*;
use log::info;

type PMsg<P> = ProMsg<P>;
type GMsg<P> = graph::GraphMsg<PMsg<P>>;

/// Represents a actor for the node in teh graph.
pub struct NodeActor<H>
where
    H: ProtocolHandler,
{
    proxies: Proxies<H::Payload>,
    ph: H,
}

impl<H> NodeActor<H>
where
    H: ProtocolHandler,
    <H as ProtocolHandler>::Payload: Debug,
{
    fn new(ph: H) -> Self {
        Self {
            proxies: Default::default(),
            ph,
        }
    }

    fn info_msg(&self, msg: &PMsg<H::Payload>) {
        let me = self.ph.aid();
        let sender = msg.sender();
        let fid = msg.from().clone();
        let to = msg.to().clone();
        let sid = *msg.sid();
        let pld = msg.payload();

        info!(
            "RECV | on {} from {} | {}->{} | {} | {:?}",
            me, sender, fid, to, sid, pld
        );
    }
}

impl<H> Actor for NodeActor<H>
where
    H: ProtocolHandler + Unpin + 'static,
{
    type Context = ::actix::Context<Self>;
}

impl<H> Handler<GMsg<H::Payload>> for NodeActor<H>
where
    H: ProtocolHandler + Unpin + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: GMsg<H::Payload>, _ctx: &mut Self::Context) -> Self::Result {
        self.proxies.handle_msg(msg);
    }
}

impl<H> Handler<PMsg<H::Payload>> for NodeActor<H>
where
    H: ProtocolHandler + Unpin + 'static,
    <H as ProtocolHandler>::Payload: Debug + Clone,
{
    type Result = ();

    fn handle(&mut self, msg: PMsg<H::Payload>, _: &mut Context<Self>) {
        self.info_msg(&msg);
        let me = self.ph.aid();

        let res = self.ph.receive(&self.proxies, msg);
        match res {
            ContinuationHandler::SendToNode(tid, msg) => {
                let from = msg.from();
                let to = msg.to();
                let sid = *msg.sid();
                let pld = msg.payload();
                info!(
                    "SEND | from {} to node {} | {}->{} | {} | {:?}",
                    me, to, from, to, sid, pld
                );

                self.proxies.do_send_to_proxy(&me, &tid, msg)
            }
            ContinuationHandler::SendToAllNodes(msg) => {
                let from = msg.from();
                let to = msg.to();
                let sid = *msg.sid();
                let pld = msg.payload();
                info!(
                    "SEND | from {} to all | {}->{} | {} | {:?}",
                    me, from, to, sid, pld
                );

                self.proxies.do_send_all_except(&me, msg, &[])
            }
            ContinuationHandler::SendToAllNodesExcept(msg, except) => {
                let from = msg.from();
                let to = msg.to();
                let sid = *msg.sid();
                let pld = msg.payload();
                info!(
                    "SEND | from {} to all-{:?} | {}->{} | {} | {:?}",
                    me, except, from, to, sid, pld
                );

                self.proxies.do_send_all_except(&me, msg, except.as_slice())
            }
            ContinuationHandler::Done => (),
        }
    }
}

impl<H> NodeActor<H>
where
    H: ProtocolHandler + Unpin,
    <H as ProtocolHandler>::Payload: Debug + Clone,
{
    /// Builds a new node actor.
    pub fn build(ph: H) -> Node<NodeActor<H>, H::Payload> {
        let aid = ph.aid();
        let actor = NodeActor::new(ph);
        let addr = NodeActor::start(actor);

        NBuilder::from_aid(aid).with_addr(addr).build()
    }
}
