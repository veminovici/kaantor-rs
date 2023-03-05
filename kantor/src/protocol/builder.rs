//! Implements the builder patter for the procotol messages.
//!
use crate::{protocol::*, ActorId};
use std::marker::PhantomData;

/// The states of the builder
pub mod states {
    /// New builder
    pub struct New {}
    /// Builder with `FromId`
    pub struct WithFromId {}
    /// Builder with `ToId`
    pub struct WithToId {}
    /// Builder with payload
    pub struct WithPayload {}
    /// Builder ready to build
    pub struct Ready {}
}

/// A builder for the protocol messages.
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

impl<P> From<ActorId> for Builder<P, states::WithFromId> {
    fn from(aid: ActorId) -> Self {
        Builder::from_aid(aid)
    }
}

impl<P> From<ProtocolMsg<P>> for Builder<P, states::WithPayload> {
    fn from(msg: ProtocolMsg<P>) -> Self {
        Builder::<P>::from_message(msg)
    }
}

impl<P> Builder<P, states::New> {
    fn new() -> Builder<P> {
        Self {
            fid: None,
            tid: None,
            payload: None,
            hid: None,
            phantom: PhantomData,
        }
    }

    /// Initializes the building chain by creating a builder from a received
    /// `Message` instance.
    pub fn from_message(msg: ProtocolMsg<P>) -> Builder<P, states::WithPayload> {
        Builder::<P, states::WithPayload> {
            fid: Some(msg.fid),
            tid: Some(msg.tid),
            payload: Some(msg.payload),
            hid: None,
            phantom: PhantomData,
        }
    }

    /// Initializes the building chain by creating a builder from an `ActorId`
    pub fn from_aid(aid: ActorId) -> Builder<P, states::WithFromId> {
        Builder::<P, states::WithFromId> {
            fid: Some(FromId::FromActor(aid)),
            tid: None,
            payload: None,
            hid: None,
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::WithFromId> {
    /// Continues the building chain by setting the `ToId` value to an actor.
    pub fn to_actor(self, aid: ActorId) -> Builder<P, states::WithToId> {
        Builder::<P, states::WithToId> {
            fid: self.fid,
            tid: Some(ToId::ToActor(aid)),
            payload: self.payload,
            hid: self.hid,
            phantom: PhantomData,
        }
    }

    /// Continues the building chain by setting the `ToId` value to all actors.
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
    /// Continues the building chain by setting the payload.
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
    /// Continues the building chain by setting the `HopId` value.
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
    /// Finalizes the chain by building the `Message` instance.
    pub fn build(self) -> ProtocolMsg<P> {
        ProtocolMsg {
            fid: self.fid.unwrap(),
            tid: self.tid.unwrap(),
            hid: self.hid.unwrap(),
            payload: self.payload.unwrap(),
        }
    }
}

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn build_() {
        let bld = Builder::from_aid(5.into());
        let bld = bld.to_actor(10.into());
        let bld = bld.with_payload(5000);
        let bld = bld.with_hid(200.into());
        let msg = bld.build();

        assert_eq!(FromId::from(5), msg.fid);
        assert_eq!(ToId::from(10), msg.tid);
        assert_eq!(HopId::from(200), msg.hid);
        assert_eq!(5000, msg.payload);

        let bld = Builder::from_message(msg);
        let bld = bld.with_hid(300.into());
        let msg = bld.build();

        assert_eq!(FromId::from(5), msg.fid);
        assert_eq!(ToId::from(10), msg.tid);
        assert_eq!(HopId::from(300), msg.hid);
        assert_eq!(5000, msg.payload);
    }
}
