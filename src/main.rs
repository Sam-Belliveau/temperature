// Include Other Files
mod sensor;
mod sensor_group;
mod lpfilter;
mod validators;

// Timing the main loop
use std::thread;
use std::time::Duration;

// Flushing to the console
use std::io::{self, Write};

// Reading temperature data
use crate::sensor_group::SensorGroup;

// Reading command line options
use clap::{Arg, App};

// Default command line options
const DEFAULT_FILTER_RC: &'static str = "2";
const DEFAULT_FILTER_ORDER: &'static str = "3";
const DEFAULT_READ_RATE: &'static str = "450";
const DEFAULT_PRINT_RATE: &'static str = "50";

fn main() {
    let matches = App::new("Sam's Temperature Reader")
        .version("0.1.0")
        .author("Sam Belliveau <sam.belliveau@gmail.com>")
        .about("Read Temperature From multiple/single HWMON Sensor Files")
        .arg(Arg::with_name("HWMON FILES")
                .index(1)
                .multiple(true)
                .takes_value(true)
                .required(true)
                .validator(validators::sensor_validator)
                .help("hwmon sensor file(s) that will be read from"))
        .arg(Arg::with_name("lowpass filter rc")
                .short("r")
                .long("filter-rc")
                .takes_value(true)
                .required(false)
                .default_value(DEFAULT_FILTER_RC)
                .validator(validators::valid_rc)
                .help("The amount that the low pass filter smooths out the temperatures"))
        .arg(Arg::with_name("lowpass filter order")
                .short("o")
                .long("filter-order")
                .takes_value(true)
                .required(false)
                .default_value(DEFAULT_FILTER_ORDER)
                .validator(validators::valid_order)
                .help("The order of the low pass filter"))
        .arg(Arg::with_name("print rate")
                .short("s")
                .long("print-ms")
                .takes_value(true)
                .required(false)
                .default_value(DEFAULT_PRINT_RATE)
                .validator(validators::valid_rate)
                .help("how many milliseconds between each print"))
        .arg(Arg::with_name("read rate")
                .long("read-ms")
                .takes_value(true)
                .required(false)
                .default_value(DEFAULT_READ_RATE)
                .validator(validators::valid_rate)
                .help("how many milliseconds between each sensor read"))
        .get_matches();

    let rrate  = matches.value_of("read rate").unwrap().parse::<u128>().unwrap();
    let prate  = matches.value_of("print rate").unwrap().parse::<u32>().unwrap();
    let rc     = matches.value_of("lowpass filter rc").unwrap().parse::<f64>().unwrap();
    let order  = matches.value_of("lowpass filter order").unwrap().parse::<usize>().unwrap();

    let files : Vec<&str> = matches.values_of("HWMON FILES").unwrap().collect();
    let mut sensors = SensorGroup::init(files, rrate, rc, order);

    loop {
        print!("\r{:.1}°", sensors.read_temp());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::new(0, 1000000 * prate));
    }
}
