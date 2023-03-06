use crate::ActorId;
use std::fmt::{Debug, Display};

/// Represents the destination of the message.
#[derive(PartialEq, Clone)]
pub enum ToId {
    /// The destination is a specific actor.
    Actor(ActorId),
    /// The destination are all actors.
    All,
    /// The destination are all actors excepts a list of them.
    AllExcept(Vec<ActorId>),
}

impl Debug for ToId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Actor(aid) => write!(f, "{aid}"),
            Self::All => write!(f, "all"),
            Self::AllExcept(e) => writeln!(f, "all-- {e:?}"),
        }
    }
}

impl Display for ToId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToId::Actor(aid) => write!(f, "{aid}"),
            ToId::All => write!(f, "all"),
            ToId::AllExcept(_) => write!(f, "all--"),
        }
    }
}

impl From<ActorId> for ToId {
    fn from(value: ActorId) -> Self {
        ToId::Actor(value)
    }
}

impl From<usize> for ToId {
    fn from(value: usize) -> Self {
        let aid = ActorId::from(value);
        Self::from(aid)
    }
}
