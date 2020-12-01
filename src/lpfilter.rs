// Time since calls is nessisary for the low pass filters
use std::time::{Instant, Duration};

// Standard struct for low pass filter
pub struct LowPassFilter {
    rc: f64,
    last_value: f64,
    last_update: Instant,
}

impl LowPassFilter {

    pub fn init(rc: f64) -> Self {
        Self {
            rc: rc,
            last_value: 0.0,
            last_update: Instant::now() - Duration::from_secs_f64(rc * 256.0)
        }
    }

    // Get filtered value
    pub fn get(&mut self, next: f64) -> f64{
        let now = Instant::now();
        let dt = (now - self.last_update).as_secs_f64();
        self.last_update = now;

        let a = (dt) / (dt + self.rc);
        self.last_value = self.last_value + a * (next - self.last_value);
        self.last_value
    }
    
}