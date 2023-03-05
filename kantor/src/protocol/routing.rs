use std::fmt::Display;

use crate::ActorId;

/// Represents origin of the message.
#[derive(Debug, PartialEq, Clone)]
pub enum FromId {
    /// The message is originating from an actor
    FromActor(ActorId),
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

/// Represents the destination of the message.
#[derive(Debug, PartialEq, Clone)]
pub enum ToId {
    /// The destination is a specific actor.
    ToActor(ActorId),
    /// The destination are all actors.
    ToAllActors,
    /// The destination are all actors excepts a list of them.
    ToAllActorsExcept(Vec<ActorId>),
}

impl Display for ToId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToId::ToActor(aid) => write!(f, "{aid}"),
            ToId::ToAllActors => write!(f, "all"),
            ToId::ToAllActorsExcept(_) => write!(f, "all--"),
        }
    }
}

impl From<ActorId> for ToId {
    fn from(value: ActorId) -> Self {
        ToId::ToActor(value)
    }
}

impl From<usize> for ToId {
    fn from(value: usize) -> Self {
        let aid = ActorId::from(value);
        Self::from(aid)
    }
}

/// Reprsents the sender of the message for this current leg.
#[derive(Debug, PartialEq, Clone)]
pub struct HopId(ActorId);

impl Display for HopId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ActorId> for HopId {
    fn from(value: ActorId) -> Self {
        Self(value)
    }
}

impl From<usize> for HopId {
    fn from(value: usize) -> Self {
        let aid = ActorId::from(value);
        Self::from(aid)
    }
}
