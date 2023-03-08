use crate::ActorId;
use std::fmt::Debug;

/// Reprsents the sender of the message for this current leg.
#[derive(PartialEq, Clone, Copy)]
pub struct SenderId(ActorId);

impl Debug for SenderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{}", self.0.inner())
    }
}

impl From<ActorId> for SenderId {
    fn from(value: ActorId) -> Self {
        Self(value)
    }
}

impl From<usize> for SenderId {
    fn from(value: usize) -> Self {
        let aid = ActorId::from(value);
        Self::from(aid)
    }
}

impl SenderId {
    /// Retrieves the actor id.
    pub fn as_aid(&self) -> ActorId {
        self.0
    }
}
