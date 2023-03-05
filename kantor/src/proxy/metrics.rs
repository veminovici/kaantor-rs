#[derive(Debug, Default)]
pub struct Metrics {
    ttl: usize,
    succ: usize,
    fail: usize,
}

impl Metrics {
    pub fn record_success(&mut self) {
        self.ttl += 1;
        self.succ += 1;
    }

    pub fn record_failure(&mut self) {
        self.ttl += 1;
        self.fail += 1;
    }
}
