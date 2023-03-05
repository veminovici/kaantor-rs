use super::ActorId;

#[derive(Debug, PartialEq)]
pub enum FromId {
    FromActor(ActorId),
}

#[derive(Debug, PartialEq)]
pub enum ToId {
    ToActor(ActorId),
    ToAllActors,
    ToAllActorsExcept(Vec<ActorId>),
}

#[derive(Debug, PartialEq)]
pub struct HopId(ActorId);

impl From<ActorId> for HopId {
    fn from(value: ActorId) -> Self {
        Self(value)
    }
}

impl From<usize> for HopId {
    fn from(value: usize) -> Self {
        let aid = ActorId::from(value);
        Self(aid)
    }
}
