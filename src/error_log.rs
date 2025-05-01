use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCode {
    WatchdogTimeout,
    Overheated,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ErrorLog {
    pub history: Vec<ErrorCode>,
}

impl ErrorLog {
    pub fn record(&mut self, code: ErrorCode) {
        self.history.push(code);
    }

    pub fn latest(&self) -> Option<&ErrorCode> {
        self.history.last()
    }
}
