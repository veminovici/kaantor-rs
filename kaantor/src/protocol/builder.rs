//! Implements the builder patter for the procotol messages.
//!
use crate::{protocol::*, ActorId};
use std::marker::PhantomData;

/// The states of the builder
mod states {
    /// New builder
    pub struct New {}
    /// Builder with `FromId`
    pub struct WithFromId {}
    /// Builder with `ToId`
    pub struct WithToId {}
    /// Builder with `SessionId`
    pub struct WithSessionId {}
    /// Builder with payload
    pub struct WithPayload {}
    /// Builder ready to build
    pub struct Ready {}
}

/// A builder for the protocol messages.
pub struct Builder<P, S = states::New> {
    fid: Option<FromId>,
    tid: Option<ToId>,
    sid: Option<SessionId>,
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
    fn new() -> Builder<P> {
        Self {
            fid: None,
            tid: None,
            sid: None,
            payload: None,
            hid: None,
            phantom: PhantomData,
        }
    }

    /// Initializes the building chain by creating a builder from a received
    /// `Message` instance.
    pub fn with_message(msg: super::Message<P>) -> Builder<P, states::WithPayload> {
        Builder::<P, states::WithPayload> {
            fid: Some(msg.fid),
            tid: Some(msg.tid),
            sid: Some(msg.sid),
            payload: Some(msg.payload),
            hid: None,
            phantom: PhantomData,
        }
    }

    /// Initializes the building chain by creating a builder from an `ActorId`
    pub fn with_from_actor(aid: ActorId) -> Builder<P, states::WithFromId> {
        Builder::<P, states::WithFromId> {
            fid: Some(FromId::Actor(aid)),
            tid: None,
            sid: None,
            payload: None,
            hid: None,
            phantom: PhantomData,
        }
    }

    /// Initializes the building chain by creating a builder from an api invocation
    pub fn with_from_api() -> Builder<P, states::WithFromId> {
        Builder::<P, states::WithFromId> {
            fid: Some(FromId::Api),
            tid: None,
            sid: None,
            payload: None,
            hid: None,
            phantom: PhantomData,
        }
    }

    /// Initializes the building chain by creating a builder initialized
    /// with the `FromId` and `ToId` values from a given message.
    pub fn with_from_to(msg: &super::Message<P>) -> Builder<P, states::WithToId> {
        let fid = msg.fid().clone();
        let tid = msg.tid().clone();
        let sid = *msg.sid();

        Builder::<P, states::WithToId> {
            fid: Some(fid),
            tid: Some(tid),
            sid: Some(sid),
            payload: None,
            hid: None,
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::WithFromId> {
    /// Continues the building chain by setting the `ToId` value to an actor.
    pub fn with_to_actor(self, aid: ActorId) -> Builder<P, states::WithToId> {
        Builder::<P, states::WithToId> {
            fid: self.fid,
            tid: Some(ToId::Actor(aid)),
            sid: self.sid,
            payload: self.payload,
            hid: self.hid,
            phantom: PhantomData,
        }
    }

    /// Continues the building chain by setting the `ToId` value to all actors.
    pub fn with_to_all_actors(self) -> Builder<P, states::WithToId> {
        Builder::<P, states::WithToId> {
            fid: self.fid,
            tid: Some(ToId::All),
            sid: self.sid,
            payload: self.payload,
            hid: self.hid,
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::WithToId> {
    /// Continues the building chain by setting the session identifier.
    pub fn with_session(self, sid: SessionId) -> Builder<P, states::WithSessionId> {
        Builder::<P, states::WithSessionId> {
            fid: self.fid,
            tid: self.tid,
            sid: Some(sid),
            payload: self.payload,
            hid: self.hid,
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::WithSessionId> {
    /// Continues the building chain by setting the payload.
    pub fn with_payload(self, payload: P) -> Builder<P, states::WithPayload> {
        Builder::<P, states::WithPayload> {
            fid: self.fid,
            tid: self.tid,
            sid: self.sid,
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
            sid: self.sid,
            payload: self.payload,
            hid: Some(hid.into()),
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::Ready> {
    /// Finalizes the chain by building the `Message` instance.
    pub fn build(self) -> super::Message<P> {
        super::Message {
            fid: self.fid.unwrap(),
            tid: self.tid.unwrap(),
            sid: self.sid.unwrap(),
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
        let msg = Builder::with_from_actor(5.into())
            .with_to_actor(10.into())
            .with_session(50.into())
            .with_payload(5000)
            .with_hid(200.into())
            .build();

        assert_eq!(FromId::from(5), msg.fid);
        assert_eq!(ToId::from(10), msg.tid);
        assert_eq!(SessionId::from(50), msg.sid);
        assert_eq!(HopId::from(200), msg.hid);
        assert_eq!(5000, msg.payload);

        let msg = Builder::with_message(msg).with_hid(300.into()).build();

        assert_eq!(FromId::from(5), msg.fid);
        assert_eq!(ToId::from(10), msg.tid);
        assert_eq!(SessionId::from(50), msg.sid);
        assert_eq!(HopId::from(300), msg.hid);
        assert_eq!(5000, msg.payload);
    }
}
