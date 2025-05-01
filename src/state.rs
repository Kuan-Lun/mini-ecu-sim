use crate::error_log::{ErrorCode, ErrorLog};
use crate::sensor::SensorInput;
use crate::watchdog::Watchdog;

#[derive(Debug)]
pub enum EngineState {
    Off,
    Idle,
    Running,
    Overheated,
    Error,
}

#[derive(Debug)]
pub struct EcuState {
    pub state: EngineState,
    watchdog: Watchdog,
    pub errors: ErrorLog,
}

impl EcuState {
    pub fn new(watchdog_timeout_secs: u64) -> Self {
        Self {
            state: EngineState::Off,
            watchdog: Watchdog::new(watchdog_timeout_secs),
            errors: ErrorLog::default(),
        }
    }

    pub fn update(&mut self, input: &impl SensorInput) {
        if self.watchdog.expired() {
            self.state = EngineState::Error;
            self.errors.record(ErrorCode::WatchdogTimeout);
            return;
        }

        let temp = input.read_temperature();
        let rpm = input.read_rpm();

        self.state = match (rpm, temp) {
            (_, t) if t > 100.0 => {
                self.errors.record(ErrorCode::Overheated);
                EngineState::Overheated
            }
            (0.0, _) => EngineState::Idle,
            (r, _) if r > 0.0 => EngineState::Running,
            _ => EngineState::Off,
        };

        // 正常流程就重設 watchdog
        if !matches!(self.state, EngineState::Error | EngineState::Overheated) {
            self.watchdog.reset();
        }
    }
}
