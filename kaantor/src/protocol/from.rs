use std::fmt::{Debug, Display};

use crate::ActorId;

/// Represents origin of the message.
#[derive(PartialEq, Clone)]
pub enum From {
    /// The message is originating from an actor
    Actor(ActorId),
    /// A public api invocation
    Api,
}

impl Debug for From {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            From::Actor(aid) => write!(f, "{aid}"),
            From::Api => write!(f, "api"),
        }
    }
}

impl Display for From {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            From::Actor(aid) => write!(f, "{aid}"),
            From::Api => write!(f, "api"),
        }
    }
}

impl core::convert::From<ActorId> for From {
    fn from(value: ActorId) -> Self {
        From::Actor(value)
    }
}

impl core::convert::From<usize> for From {
    fn from(value: usize) -> Self {
        let aid = ActorId::from(value);
        Self::from(aid)
    }
}
