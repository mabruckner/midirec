use std::thread::sleep;
use std::time::Duration;

use std::io::prelude::*;
use std::io;
use std::fs::File;

fn export(pin: usize) -> Result<(), io::Error> {
    let mut file = try!{File::open("/sys/class/gpio/export")};
    try!{write!(file, "{}", pin)};
    Ok(())
}

fn unexport(pin: usize) -> Result<(), io::Error> {
    let mut file = try!{File::open("/sys/class/gpio/unexport")};
    try!{write!(file, "{}", pin)};
    Ok(())
}

fn write_val(pin: usize, value: usize) -> Result<(), io::Error> {
    let mut file = try!{File::open(format!("/sys/class/gpio/{}/value", pin))};
    try!{write!(file, "{}", value)};
    Ok(())
}

fn main() {
    export(17).unwrap();
    write_val(17, 1).unwrap();
    sleep(Duration::from_millis(1000));
    write_val(17, 0).unwrap();
    unexport(17).unwrap();
}
