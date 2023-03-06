use std::fmt::{Debug, Display};

use crate::ActorId;

/// Represents origin of the message.
#[derive(PartialEq, Clone)]
pub enum FromId {
    /// The message is originating from an actor
    FromActor(ActorId),
}

impl Debug for FromId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromId::FromActor(aid) => write!(f, "{aid}"),
        }
    }
}

impl Display for FromId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromId::FromActor(aid) => write!(f, "{aid}"),
        }
    }
}

impl From<ActorId> for FromId {
    fn from(value: ActorId) -> Self {
        FromId::FromActor(value)
    }
}

impl From<usize> for FromId {
    fn from(value: usize) -> Self {
        let aid = ActorId::from(value);
        Self::from(aid)
    }
}
