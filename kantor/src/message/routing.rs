use super::ActorId;

#[derive(Debug, PartialEq)]
pub enum FromId {
    FromActor(ActorId),
}

impl From<ActorId> for FromId {
    fn from(value: ActorId) -> Self {
        FromId::FromActor(value)
    }
}

impl From<usize> for FromId {
    fn from(value: usize) -> Self {
        let aid = ActorId::from(value);
        Self::from(aid)
    }
}

#[derive(Debug, PartialEq)]
pub enum ToId {
    ToActor(ActorId),
    ToAllActors,
    ToAllActorsExcept(Vec<ActorId>),
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
        Self::from(aid)
    }
}
