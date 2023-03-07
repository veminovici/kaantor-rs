use std::fmt::{Debug, Display};

use crate::ActorId;

/// Represents origin of the message.
#[derive(PartialEq, Clone)]
pub enum FromId {
    /// The message is originating from an actor
    Actor(ActorId),
    /// A public api invocation
    Api,
}

impl Debug for FromId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromId::Actor(aid) => write!(f, "{aid}"),
            FromId::Api => write!(f, "api"),
        }
    }
}

impl Display for FromId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromId::Actor(aid) => write!(f, "{aid}"),
            FromId::Api => write!(f, "api"),
        }
    }
}

impl From<ActorId> for FromId {
    fn from(value: ActorId) -> Self {
        FromId::Actor(value)
    }
}

impl From<usize> for FromId {
    fn from(value: usize) -> Self {
        let aid = ActorId::from(value);
        Self::from(aid)
    }
}
