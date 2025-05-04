mod error_log;
mod sensor;
mod state;
mod watchdog;

use std::fs::File;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use sensor::MockSensor;
use state::EcuState;

fn main() {
    let mut ecu = EcuState::new(2);

    let mock_sensor = MockSensor {
        temp: 25.0,
        rpm: 0.0,
    };

    ecu.update(&mock_sensor);
    println!("ECU State (1st update): {:?}", ecu.state);

    sleep(Duration::from_secs(3));
    ecu.update(&mock_sensor);
    println!("ECU State (2nd update): {:?}", ecu.state);

    // 印出 JSON
    let json = serde_json::to_string_pretty(&ecu.errors).unwrap();
    println!("Error log in JSON:\n{}", json);

    // 選擇性寫入檔案
    let mut file = File::create("errors.json").unwrap();
    writeln!(file, "{}", json).unwrap();

    if let Some(latest) = ecu.errors.latest() {
        println!("Most recent error: {:?}", latest);
    }
}
