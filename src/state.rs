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
}

impl EcuState {
    pub fn new() -> Self {
        Self {
            state: EngineState::Off,
        }
    }

    pub fn update(&mut self, input: &impl SensorInput, watchdog: &mut Watchdog) {
        if watchdog.expired() {
            self.state = EngineState::Error;
            return;
        }

        let temp = input.read_temperature();
        let rpm = input.read_rpm();

        self.state = match (rpm, temp) {
            (_, t) if t > 100.0 => EngineState::Overheated,
            (0.0, _) => EngineState::Idle,
            (r, _) if r > 0.0 => EngineState::Running,
            _ => EngineState::Off,
        };

        // 若狀態正常，代表有在運作，重設 watchdog
        if !matches!(self.state, EngineState::Error | EngineState::Overheated) {
            watchdog.reset();
        }
    }
}
