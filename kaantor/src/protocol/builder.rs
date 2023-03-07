//! Implements the builder patter for the procotol messages.
//!
use crate::{protocol::*, ActorId};
use std::marker::PhantomData;

/// The states of the builder
mod states {
    /// New builder
    pub struct New {}
    /// Builder with `FromId`
    pub struct WithFrom {}
    /// Builder with `ToId`
    pub struct WithTo {}
    /// Builder with `SessionId`
    pub struct WithSessionId {}
    /// Builder with payload
    pub struct WithPayload {}
    /// Builder ready to build
    pub struct Ready {}
}

/// A builder for the protocol messages.
pub struct Builder<P, S = states::New> {
    from: Option<From>,
    to: Option<To>,
    sid: Option<SessionId>,
    sender: Option<SenderId>,
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
            from: None,
            to: None,
            sid: None,
            payload: None,
            sender: None,
            phantom: PhantomData,
        }
    }

    /// Initializes the building chain by creating a builder from a received
    /// `Message` instance.
    pub fn with_message(msg: super::Message<P>) -> Builder<P, states::WithPayload> {
        Builder::<P, states::WithPayload> {
            from: Some(msg.from),
            to: Some(msg.to),
            sid: Some(msg.sid),
            payload: Some(msg.payload),
            sender: None,
            phantom: PhantomData,
        }
    }

    /// Initializes the building chain by creating a builder from an `ActorId`
    pub fn with_from_actor(aid: ActorId) -> Builder<P, states::WithFrom> {
        Builder::<P, states::WithFrom> {
            from: Some(From::from(aid)),
            to: None,
            sid: None,
            payload: None,
            sender: None,
            phantom: PhantomData,
        }
    }

    /// Initializes the building chain by creating a builder from an api invocation
    pub fn with_from_api() -> Builder<P, states::WithFrom> {
        Builder::<P, states::WithFrom> {
            from: Some(From::Api),
            to: None,
            sid: None,
            payload: None,
            sender: None,
            phantom: PhantomData,
        }
    }

    /// Initializes the building chain by creating a builder initialized
    /// with the `FromId` and `ToId` values from a given message.
    pub fn with_from_to(msg: &super::Message<P>) -> Builder<P, states::WithTo> {
        let from = msg.from().clone();
        let to = msg.to().clone();
        let sid = *msg.sid();

        Builder::<P, states::WithTo> {
            from: Some(from),
            to: Some(to),
            sid: Some(sid),
            payload: None,
            sender: None,
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::WithFrom> {
    /// Continues the building chain by setting the `ToId` value to an actor.
    pub fn with_to_actor(self, aid: ActorId) -> Builder<P, states::WithTo> {
        Builder::<P, states::WithTo> {
            from: self.from,
            to: Some(To::from(aid)),
            sid: self.sid,
            payload: self.payload,
            sender: self.sender,
            phantom: PhantomData,
        }
    }

    /// Continues the building chain by setting the `ToId` value to all actors.
    pub fn with_to_all_actors(self) -> Builder<P, states::WithTo> {
        Builder::<P, states::WithTo> {
            from: self.from,
            to: Some(To::All),
            sid: self.sid,
            payload: self.payload,
            sender: self.sender,
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::WithTo> {
    /// Continues the building chain by setting the session identifier.
    pub fn with_session(self, sid: SessionId) -> Builder<P, states::WithSessionId> {
        Builder::<P, states::WithSessionId> {
            from: self.from,
            to: self.to,
            sid: Some(sid),
            payload: self.payload,
            sender: self.sender,
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::WithSessionId> {
    /// Continues the building chain by setting the payload.
    pub fn with_payload(self, payload: P) -> Builder<P, states::WithPayload> {
        Builder::<P, states::WithPayload> {
            from: self.from,
            to: self.to,
            sid: self.sid,
            payload: Some(payload),
            sender: self.sender,
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::WithPayload> {
    /// Continues the building chain by setting the `HopId` value.
    pub fn with_sender(self, sender: ActorId) -> Builder<P, states::Ready> {
        Builder::<P, states::Ready> {
            from: self.from,
            to: self.to,
            sid: self.sid,
            payload: self.payload,
            sender: Some(sender.into()),
            phantom: PhantomData,
        }
    }
}

impl<P> Builder<P, states::Ready> {
    /// Finalizes the chain by building the `Message` instance.
    pub fn build(self) -> super::Message<P> {
        super::Message {
            from: self.from.unwrap(),
            to: self.to.unwrap(),
            sid: self.sid.unwrap(),
            sender: self.sender.unwrap(),
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
            .with_sender(200.into())
            .build();

        assert_eq!(From::from(5), msg.from);
        assert_eq!(To::from(10), msg.to);
        assert_eq!(SessionId::from(50), msg.sid);
        assert_eq!(SenderId::from(200), msg.sender);
        assert_eq!(5000, msg.payload);

        let msg = Builder::with_message(msg).with_sender(300.into()).build();

        assert_eq!(From::from(5), msg.from);
        assert_eq!(To::from(10), msg.to);
        assert_eq!(SessionId::from(50), msg.sid);
        assert_eq!(SenderId::from(300), msg.sender);
        assert_eq!(5000, msg.payload);
    }
}
