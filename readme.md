# Smooth Temperature Sensor

## To Compile

Because this is a rust project, you can just run:

```sh
cargo build --release
```

## How to install 

### Arch Linux

To install this program just run:

```sh
wget https://github.com/Sam-Belliveau/temperature/releases/download/v0.1.0/smooth-temperature-bin-0.1.0-1-x86_64.pkg.tar.xz
sudo pacman -U ./smooth-temperature-bin-0.1.0-1-x86_64.pkg.tar.xz
```

## How to use

This is what the help page looks like:
```sh
─╯ smooth-temperature --help
Sam's Temperature Reader 0.1.0
Sam Belliveau <sam.belliveau@gmail.com>
Read Temperature From hwmon Sensor Files

USAGE:
    temperature [OPTIONS] <HWMON FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --filter-order <lowpass filter order>    The order of the low pass filter [default: 3]
    -r, --filter-rc <lowpass filter rc>
            The amount that the low pass filter smooths out the temperatures [default: 2]

    -s, --print-ms <print rate>                  how many milliseconds between each print [default: 50]
        --read-ms <read rate>                    how many milliseconds between each sensor read [default: 450]

ARGS:
    <HWMON FILE>    hwmon sensor file that will be read from
```

The default values are usually good enough.

The way this works is every 50ms, it prints out the temperature of the hwmon file that you give it. Even though the hwmon file only updates every ~1005ms, this program will interpolate between values, making the temperature appear smoother.

An example of using this program would look like:
```sh
─╯  smooth-temperature /sys/devices/platform/coretemp.0/hwmon/hwmon2/temp1_input 
```

## Using in polybar

For this module, I am assuming you've copied the temperature binary to your home directory, you dont have to put it here, you can put it in your /usr/bin if you want.

```ini
[module/temperature]
type = custom/script
exec = smooth-temperature /sys/devices/platform/coretemp.0/hwmon/hwmon2/temp1_input 

interval = 0.05
tail = true 

format-prefix = "Temp: "
```

