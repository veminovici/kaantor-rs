use std::fmt::Display;

use crate::ActorId;

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

impl HopId {
    /// Retrieves the actor id.
    pub fn aid(&self) -> ActorId {
        self.0
    }
}
