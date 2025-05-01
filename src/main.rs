mod sensor;
mod state;
mod watchdog;

use std::thread::sleep;
use std::time::Duration;

use sensor::MockSensor;
use state::EcuState;
use watchdog::Watchdog;

fn main() {
    let mut ecu = EcuState::new();

    let mock_sensor = MockSensor {
        temp: 25.0,
        rpm: 0.0,
    };

    let mut wd = Watchdog::new(2); // 模擬 2 秒超時

    // 模擬正常狀況下更新
    ecu.update(&mock_sensor, &mut wd);
    println!("Current ECU State: {:?}", ecu);

    // 模擬過了 3 秒未重設 watchdog（應該過期）
    sleep(Duration::from_secs(3));
    ecu.update(&mock_sensor, &mut wd);
    println!("Current ECU State after 3s: {:?}", ecu);
}
