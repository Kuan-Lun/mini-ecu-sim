mod error_log;
mod sensor;
mod state;
mod watchdog;

use sensor::MockSensor;
use state::EcuState;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut ecu = EcuState::new(2); // watchdog timeout = 2 秒

    let mock_sensor = MockSensor {
        temp: 25.0,
        rpm: 0.0,
    };

    ecu.update(&mock_sensor);
    println!("ECU State (1st update): {:?}", ecu.state);

    sleep(Duration::from_secs(3)); // 模擬掛住了
    ecu.update(&mock_sensor);
    println!("ECU State (2nd update): {:?}", ecu.state);

    println!("Error log: {:?}", ecu.errors.history);

    if let Some(code) = ecu.errors.latest() {
        println!("Latest error: {:?}", code);
    }
}
