//! Main functionality for the protocol messages.

pub mod builder;
mod routing;

use actix::prelude::*;
pub use routing::*;
pub use std::fmt::Debug;

/// A protocol message
#[derive(Clone)]
pub struct ProtocolMsg<P> {
    pub(crate) fid: FromId,
    pub(crate) tid: ToId,
    pub(crate) hid: HopId,
    pub(crate) payload: P,
}

impl<P> Debug for ProtocolMsg<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("fid", &self.fid)
            .field("tid", &self.tid)
            .field("hid", &self.hid)
            .field("payload", &"------")
            .finish()
    }
}

impl<P> ProtocolMsg<P> {
    /// Get the `FromId`
    pub fn fid(&self) -> &FromId {
        &self.fid
    }

    /// Get the `ToId`
    pub fn tid(&self) -> &ToId {
        &self.tid
    }

    /// Get the `HopId`
    pub fn hid(&self) -> &HopId {
        &self.hid
    }

    /// Get the payload.
    pub fn payload(&self) -> &P {
        &self.payload
    }
}

impl<P> Message for ProtocolMsg<P> {
    type Result = ();
}
