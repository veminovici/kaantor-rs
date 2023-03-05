//! Graph of nodes
//!
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
