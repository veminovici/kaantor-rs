//! Main functionality for the protocol messages.

mod builder;
mod from;
mod sender;
mod session;
mod to;

pub use builder::*;
pub use from::*;
pub use sender::*;
pub use session::*;
pub use to::*;

use std::fmt::Debug;

/// A protocol message
#[derive(Clone)]
pub struct Message<P> {
    pub(crate) from: From,
    pub(crate) to: To,
    pub(crate) session: Session,
    pub(crate) sender: SenderId,
    pub(crate) payload: P,
}

impl<P> Debug for Message<P>
where
    P: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] [{}-->{}] [{}]",
            &self.session, &self.from, &self.to, &self.sender
        )
    }
}

impl<P> Message<P> {
    /// Get the `FromId`
    pub fn from(&self) -> &From {
        &self.from
    }

    /// Get the `ToId`
    pub fn to(&self) -> &To {
        &self.to
    }

    /// Get the `SessionId`
    pub fn session(&self) -> &Session {
        &self.session
    }

    /// Get the `HopId`
    pub fn sender(&self) -> &SenderId {
        &self.sender
    }

    /// Get the payload.
    pub fn payload(&self) -> &P {
        &self.payload
    }
}

impl<P> actix::Message for Message<P> {
    type Result = ();
}
