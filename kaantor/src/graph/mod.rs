//! Graph of nodes
//!
use std::fmt::Display;

use actix::prelude::*;

use crate::proxy::Proxy;

/// Represents the configuration message
/// which can be send to the nodes to configure
/// the connectivity graph.
#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum GraphMsg<M>
where
    M: Message + Send,
    M::Result: Send,
{
    /// Adds a new proxy which represents the connection to the remote node.
    AddProxy(Proxy<M>),
}

impl<M> Display for GraphMsg<M>
where
    M: Message + Send,
    M::Result: Send,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphMsg::AddProxy(pxy) => write!(f, "add {}", pxy.aid),
        }
    }
}
