use std::fmt::{Debug, Display};

/// Represents a unique session
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SessionId(usize);

impl Debug for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for SessionId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
