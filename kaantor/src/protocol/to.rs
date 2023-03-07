use crate::ActorId;
use std::fmt::{Debug, Display};

/// Represents the destination of the message.
#[derive(PartialEq, Clone)]
pub enum To {
    /// The destination is a specific actor.
    Actor(ActorId),
    /// The destination are all actors.
    All,
    /// The destination are all actors excepts a list of them.
    AllExcept(Vec<ActorId>),
}

impl Debug for To {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Actor(aid) => write!(f, "{aid}"),
            Self::All => write!(f, "all"),
            Self::AllExcept(e) => writeln!(f, "all-- {e:?}"),
        }
    }
}

impl Display for To {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            To::Actor(aid) => write!(f, "{aid}"),
            To::All => write!(f, "all"),
            To::AllExcept(_) => write!(f, "all--"),
        }
    }
}

impl From<ActorId> for To {
    fn from(value: ActorId) -> Self {
        To::Actor(value)
    }
}

impl From<usize> for To {
    fn from(value: usize) -> Self {
        let aid = ActorId::from(value);
        Self::from(aid)
    }
}
