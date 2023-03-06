/// Represents a unique session
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionId(usize);

impl From<usize> for SessionId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
