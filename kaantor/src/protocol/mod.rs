//! Main functionality for the protocol messages.

mod builder;
mod fid;
mod sender;
mod sid;
mod tid;

pub use builder::*;
pub use fid::*;
pub use sender::*;
pub use sid::*;
pub use tid::*;

use std::fmt::Debug;

/// A protocol message
#[derive(Clone)]
pub struct Message<P> {
    pub(crate) fid: FromId,
    pub(crate) tid: ToId,
    pub(crate) sid: SessionId,
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
            &self.sid, &self.fid, &self.tid, &self.sender
        )
    }
}

impl<P> Message<P> {
    /// Get the `FromId`
    pub fn fid(&self) -> &FromId {
        &self.fid
    }

    /// Get the `ToId`
    pub fn tid(&self) -> &ToId {
        &self.tid
    }

    /// Get the `SessionId`
    pub fn sid(&self) -> &SessionId {
        &self.sid
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
