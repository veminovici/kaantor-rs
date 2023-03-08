use std::fmt::Debug;

/// Represents a unique session
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Session(usize);

impl Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "K{}", self.0)
    }
}

impl From<usize> for Session {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
