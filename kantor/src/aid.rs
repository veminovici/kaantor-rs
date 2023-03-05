use std::fmt::Display;

/// The identifier for an actor
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ActorId(usize);

impl Display for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for ActorId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
