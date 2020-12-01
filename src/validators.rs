use crate::sensor::read_temp_from_file;

// These check if command line arguments are valid

pub fn sensor_validator(file: String) -> Result<(), String> {
    match read_temp_from_file(&file) {
        Ok(_temp) => Ok(()),
        Err(err) => Err(err)
    }
}

pub fn valid_rate(rate: String) -> Result<(), String> {
    match rate.parse::<u32>() {
        Ok(_rate) => Ok(()),
        Err(_err) => Err("invalid value for update rate".to_string())
    }
}

pub fn valid_rc(rc: String) -> Result<(), String> {
    match rc.parse::<f64>() {
        Ok(rate) => {
            if rate >= 0.0 {
                Ok(())
            } else {
                Err("value for rc must be greater than or equal 0".to_string())
            }
        },
        Err(_err) => Err("invalid value for rc".to_string())
    }
}