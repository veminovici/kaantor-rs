use std::fmt::Debug;

/// The identifier for an actor
#[derive(PartialEq, Clone, Copy)]
pub struct ActorId(usize);

impl Debug for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A{}", self.0)
    }
}

impl ActorId {
    pub(crate) fn inner(&self) -> usize {
        self.0
    }
}

impl From<usize> for ActorId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
