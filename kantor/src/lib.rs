#![warn(missing_docs)]

//! A crate for distributed systems

mod aid;
pub mod graph;
pub mod node;
pub mod protocol;
mod proxy;

pub use aid::*;
pub use node::{Node, Proxies};

use actix::{dev::ToEnvelope, prelude::*};
use protocol::Message as PMsg;

type GMsg<P> = graph::GraphMsg<PMsg<P>>;

/// Add a bi-directional connection between two ndoes.
pub async fn add_edge<A, P>(a: &mut Node<A, P>, b: &mut Node<A, P>)
where
    P: Send + 'static,
    A: Actor,
    A: Handler<PMsg<P>>,
    A::Context: ToEnvelope<A, PMsg<P>>,
    A: Handler<GMsg<P>>,
    A::Context: ToEnvelope<A, GMsg<P>>,
{
    let pxy_a = a.as_proxy();
    let pxy_b = b.as_proxy();

    let _ = a.add_proxy(pxy_b).await;
    let _ = b.add_proxy(pxy_a).await;
}
