// Filter used to smooth out data
use crate::lpfilter::LowPassFilter;

// Used to check the time since last update
use std::time::{Instant, Duration};

// Read a temperature from a file and report an error if you cant
pub fn read_temp_from_file(file: &str) -> Result<f64, String> {
    use std::fs;
    if let Ok(temp_str) = fs::read_to_string(file) {
        if let Ok(temp_int) = temp_str.trim().parse::<i64>() {
            Ok((temp_int as f64) / (1000.0))
        } else {
            Err("error parsing temperature from supplied hwmon sensor file".to_string())
        }
    } else {
        Err(format!("error opening hwmon sensor file [{}]", file))
    }
}

// Struct used to measure sensors
#[derive(Clone)]
pub struct Sensor {
    sensor_file: String,
    update_ms: u128,

    last_temp: f64,
    last_update: Instant,

    filters: Vec<LowPassFilter>
}

#[warn(dead_code)]
impl Sensor {

    pub fn init(file: &str, update_ms: u128, filter_rc: f64, filter_order: usize) -> Self {
        Self {
            sensor_file: file.to_string(),
            update_ms: update_ms,

            last_temp: 0.0,
            last_update: Instant::now() - Duration::from_millis((2 * update_ms) as u64),

            filters: {
                let mut filters = Vec::new();
                for _i in 0..filter_order {
                    filters.push(LowPassFilter::init(filter_rc / (filter_order as f64)));
                }
                filters
            }
        }
    }

    // Force read without checking time since update
    pub fn force_raw_read_temp(&self) -> f64 {
        read_temp_from_file(&self.sensor_file).unwrap()
    }

    // Read temperature without filtering
    pub fn raw_read_temp(&mut self) -> f64 {
        let now = Instant::now();

        if self.update_ms < (now - self.last_update).as_millis() {
            self.last_update = now;

            let temp = self.force_raw_read_temp();
            self.last_temp = temp;
        }

        self.last_temp
    }

    // Read temperature normally
    pub fn read_temp(&mut self) -> f64 {
        let mut temp = self.raw_read_temp();
        for filter in self.filters.iter_mut() {
            temp = filter.get(temp);
        }
        temp
    }
}
