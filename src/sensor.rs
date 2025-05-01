pub trait SensorInput {
    fn read_temperature(&self) -> f32;
    fn read_rpm(&self) -> f32;
}

pub struct MockSensor {
    pub temp: f32,
    pub rpm: f32,
}

impl Default for MockSensor {
    fn default() -> Self {
        Self {
            temp: 25.0,
            rpm: 0.0,
        }
    }
}

impl SensorInput for MockSensor {
    fn read_temperature(&self) -> f32 {
        self.temp
    }

    fn read_rpm(&self) -> f32 {
        self.rpm
    }
}
