use mini_ecu_sim::{
    sensor::MockSensor,
    state::{EcuState, EngineState},
};

#[test]
fn test_state_transitions() {
    let mut ecu = EcuState::new();

    let mut sensor = MockSensor {
        temp: 25.0,
        rpm: 0.0,
    };
    ecu.update(&sensor);
    assert_eq!(matches!(ecu.state, EngineState::Idle), true);

    sensor.rpm = 2000.0;
    ecu.update(&sensor);
    assert_eq!(matches!(ecu.state, EngineState::Running), true);

    sensor.temp = 150.0;
    ecu.update(&sensor);
    assert_eq!(matches!(ecu.state, EngineState::Overheated), true);
}
