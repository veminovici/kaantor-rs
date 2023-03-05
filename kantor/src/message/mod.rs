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

#[cfg(test)]
mod utests {
    use crate::message::builder::Builder;

    use super::*;

    #[test]
    fn build_() {
        let bld = Builder::<usize>::new();
        let bld = bld.from_actor(5.into());
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
