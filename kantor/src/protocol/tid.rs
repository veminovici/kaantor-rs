use crate::ActorId;
use std::fmt::Display;

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
