pub mod builder;
mod routing;

pub use routing::*;
pub use std::fmt::Debug;

#[derive(Clone)]
pub struct Message<P> {
    pub(crate) fid: FromId,
    pub(crate) tid: ToId,
    pub(crate) hid: HopId,
    pub(crate) payload: P,
}

impl<P> Debug for Message<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("fid", &self.fid)
            .field("tid", &self.tid)
            .field("hid", &self.hid)
            .field("payload", &"------")
            .finish()
    }
}

impl<P> Message<P> {
    pub fn fid(&self) -> &FromId {
        &self.fid
    }

    pub fn tid(&self) -> &ToId {
        &self.tid
    }

    pub fn hid(&self) -> &HopId {
        &self.hid
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }
}

impl<P> actix::Message for Message<P> {
    type Result = ();
}

