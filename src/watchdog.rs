use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Watchdog {
    last_reset: Instant,
    timeout: Duration,
}

impl Watchdog {
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            last_reset: Instant::now(),
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    pub fn reset(&mut self) {
        self.last_reset = Instant::now();
    }

    pub fn expired(&self) -> bool {
        Instant::now().duration_since(self.last_reset) > self.timeout
    }
}
