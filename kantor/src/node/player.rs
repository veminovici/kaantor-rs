use crate::{node::Builder as NBuilder, protocol::Message as ProMsg, *};
use actix::prelude::*;

type PMsg<P> = ProMsg<P>;
type GMsg<P> = graph::GraphMsg<PMsg<P>>;

pub trait PlayerHandler {
    type Payload: Send;

    fn aid(&self) -> ActorId;

    fn handler(
        &mut self,
        proxies: &mut Proxies<Self::Payload>,
        msg: protocol::Message<Self::Payload>,
    );
}

pub struct Player<H>
where
    H: PlayerHandler,
{
    aid: ActorId,
    proxies: Proxies<H::Payload>,
    ph: H,
}

impl<H> Player<H>
where
    H: PlayerHandler,
{
    pub fn new(aid: ActorId, ph: H) -> Self {
        Self {
            aid,
            proxies: Default::default(),
            ph,
        }
    }
}

impl<H> Actor for Player<H>
where
    H: PlayerHandler + Unpin + 'static,
{
    type Context = ::actix::Context<Self>;
}

impl<H> Player<H>
where
    H: PlayerHandler + Unpin,
{
    pub fn build(ph: H) -> Node<Player<H>, H::Payload> {
        let aid = ph.aid();
        let actor = Player::new(aid.clone(), ph);
        let addr = Player::start(actor);

        NBuilder::from_aid(aid).with_addr(addr).build()
    }
}

impl<H> Handler<GMsg<H::Payload>> for Player<H>
where
    H: PlayerHandler + Unpin + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: GMsg<H::Payload>, _ctx: &mut Self::Context) -> Self::Result {
        //println!("Actor {:?} received a graph {:?} message", self.aid, msg);
        self.proxies.handle_msg(msg);
    }
}

impl<H> Handler<PMsg<H::Payload>> for Player<H>
where
    H: PlayerHandler + Unpin + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: PMsg<H::Payload>, _: &mut Context<Self>) {
        self.ph.handler(&mut self.proxies, msg)
    }
}
