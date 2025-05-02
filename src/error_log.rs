use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCode {
    WatchdogTimeout,
    Overheated,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorEntry {
    pub code: ErrorCode,
    pub timestamp: String, // ISO 8601 格式的字串
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ErrorLog {
    pub history: Vec<ErrorEntry>,
}

impl ErrorLog {
    pub fn record(&mut self, code: ErrorCode) {
        let timestamp = current_timestamp();
        self.history.push(ErrorEntry { code, timestamp });
    }

    pub fn latest(&self) -> Option<&ErrorEntry> {
        self.history.last()
    }
}

/// 取得目前時間的 ISO 8601 格式
fn current_timestamp() -> String {
    let now = chrono::Utc::now();
    now.to_rfc3339()
}
