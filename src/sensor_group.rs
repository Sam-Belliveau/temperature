use crate::sensor::Sensor;

#[derive(Clone)]
pub struct SensorGroup {
    sensors: Vec<Sensor>,
}

impl SensorGroup {

    pub fn init(files: Vec<&str>, update_ms: u128, filter_rc: f64, filter_order: usize) -> Self {
        Self {
            sensors: {
                let mut sensors : Vec<Sensor> = vec![];

                for f in files {
                    sensors.push(Sensor::init(f, update_ms, filter_rc, filter_order));
                }

                sensors
            }
        }
    }

    // Force read without checking time since update
    pub fn force_raw_read_temp(&self) -> f64 {
        let mut total : f64 = 0.0;

        for sensor in &self.sensors {
            total += sensor.force_raw_read_temp();
        }

        total / self.sensors.len() as f64
    }

    // Read temperature without filtering
    pub fn raw_read_temp(&mut self) -> f64 {
        let mut total : f64 = 0.0;

        for sensor in &mut self.sensors {
            total += sensor.raw_read_temp();
        }

        total / self.sensors.len() as f64
    }

    // Read temperature normally
    pub fn read_temp(&mut self) -> f64 {
        let mut total : f64 = 0.0;

        for sensor in &mut self.sensors {
            total += sensor.read_temp();
        }

        total / self.sensors.len() as f64
    }
}