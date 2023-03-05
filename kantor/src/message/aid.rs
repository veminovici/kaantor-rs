#[derive(Debug, PartialEq)]
pub struct ActorId(usize);

impl From<usize> for ActorId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
