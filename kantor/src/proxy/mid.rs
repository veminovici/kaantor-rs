#[derive(Debug)]
pub(crate) struct MessageId(usize);

impl Default for MessageId {
    fn default() -> Self {
        Self::new(0)
    }
}

impl MessageId {
    pub(crate) fn new(v: usize) -> Self {
        Self(v)
    }

    pub(crate) fn increment_mid(&mut self) -> MessageId {
        let mid = self.0;
        self.0 += 1;
        Self(mid)
    }
}
