
use crate::{message::*, ActorId};
use std::marker::PhantomData;

mod states {
    pub struct New {}
    pub struct WithFromId {}
    pub struct WithToId {}
    pub struct WithPayload {}
    pub struct Ready {}
}

pub struct Builder<P, S = states::New> {
    fid: Option<FromId>,
    tid: Option<ToId>,
    hid: Option<HopId>,
    payload: Option<P>,
    phantom: PhantomData<S>,
}

impl<P> Default for Builder<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P> Builder<P, states::New> {
    pub fn new() -> Builder<P> {
        Self {
            fid: None,
            tid: None,
            payload: None,
            hid: None,
            phantom: PhantomData,
        }
    }

    pub fn from_message(msg: Message<P>) -> Builder<P, states::WithPayload> {
        Builder::<P, states::WithPayload> {
            fid: Some(msg.fid),
            tid: Some(msg.tid),
            payload: Some(msg.payload),
            hid: None,
            phantom: PhantomData,
        }
    }

    pub fn from_actor(self, aid: ActorId) -> Builder<P, states::WithFromId> {
        Builder::<P, states::WithFromId> {
            fid: Some(FromId::FromActor(aid)),
            tid: self.tid,
            payload: self.payload,
            hid: self.hid,
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::WithFromId> {
    pub fn to_actor(self, aid: ActorId) -> Builder<P, states::WithToId> {
        Builder::<P, states::WithToId> {
            fid: self.fid,
            tid: Some(ToId::ToActor(aid)),
            payload: self.payload,
            hid: self.hid,
            phantom: PhantomData,
        }
    }

    pub fn to_all_actors(self) -> Builder<P, states::WithToId> {
        Builder::<P, states::WithToId> {
            fid: self.fid,
            tid: Some(ToId::ToAllActors),
            payload: self.payload,
            hid: self.hid,
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::WithToId> {
    pub fn with_payload(self, payload: P) -> Builder<P, states::WithPayload> {
        Builder::<P, states::WithPayload> {
            fid: self.fid,
            tid: self.tid,
            payload: Some(payload),
            hid: self.hid,
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::WithPayload> {
    pub fn with_hid(self, hid: ActorId) -> Builder<P, states::Ready> {
        Builder::<P, states::Ready> {
            fid: self.fid,
            tid: self.tid,
            payload: self.payload,
            hid: Some(hid.into()),
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::Ready> {
    pub fn build(self) -> Message<P> {
        Message {
            fid: self.fid.unwrap(),
            tid: self.tid.unwrap(),
            hid: self.hid.unwrap(),
            payload: self.payload.unwrap(),
        }
    }
}
