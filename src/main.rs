// Include Other Files
mod sensor;
mod lpfilter;
mod validators;

// Timing the main loop
use std::thread;
use std::time::Duration;

// Reading temperature data
use crate::sensor::Sensor;

// Reading command line options
use clap::{Arg, App};

// Default command line options
const DEFAULT_FILTER_RC: &'static str = "1.0";
const DEFAULT_READ_RATE: &'static str = "350";
const DEFAULT_PRINT_RATE: &'static str = "25";

fn main() {
    let matches = App::new("Sam's Temperature Reader")
        .version("0.1.0")
        .author("Sam Belliveau <sam.belliveau@gmail.com>")
        .about("Read Temperature From hwmon Sensor Files")
        .arg(Arg::with_name("HWMON FILE")
                 .index(1)
                 .takes_value(true)
                 .required(true)
                 .validator(validators::sensor_validator)
                 .help("hwmon sensor file that will be read from"))
        .arg(Arg::with_name("read rate")
                 .short("r")
                 .long("read-rate")
                 .takes_value(true)
                 .required(false)
                 .default_value(DEFAULT_READ_RATE)
                 .validator(validators::valid_rate)
                 .help("how many milliseconds between each sensor read"))
        .arg(Arg::with_name("print rate")
                 .short("p")
                 .long("print-rate")
                 .takes_value(true)
                 .required(false)
                 .default_value(DEFAULT_PRINT_RATE)
                 .validator(validators::valid_rate)
                 .help("how many milliseconds between each print"))
        .arg(Arg::with_name("lowpass rc")
                 .short("f")
                 .long("filter-rc")
                 .takes_value(true)
                 .required(false)
                 .default_value(DEFAULT_FILTER_RC)
                 .validator(validators::valid_rc)
                 .help("The amount that the low pass filter smooths out the temperatures"))
        .get_matches();

    let file = matches.value_of("HWMON FILE").unwrap().to_string();
    let rrate = matches.value_of("read rate").unwrap().parse::<u128>().unwrap();
    let prate = matches.value_of("print rate").unwrap().parse::<u32>().unwrap();
    let rc   = matches.value_of("lowpass rc").unwrap().parse::<f64>().unwrap();

    let mut s = Sensor::init(file, rrate, rc);

    loop {
        println!("{:.1}°", s.read_temp());
        thread::sleep(Duration::new(0, 1000000 * prate));
    }
}
