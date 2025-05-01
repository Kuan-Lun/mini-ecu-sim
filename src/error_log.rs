#[derive(Debug)]
pub enum ErrorCode {
    WatchdogTimeout,
    Overheated,
}

#[derive(Debug, Default)]
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
