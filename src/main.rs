use std::thread::sleep;
use std::time::Duration;

use std::io::prelude::*;
use std::io;
use std::fs::File;

fn export(pin: usize) -> Result<(), io::Error> {
    let mut file = try!{File::create("/sys/class/gpio/export")};
    try!{write!(file, "{}", pin)};
    Ok(())
}

fn unexport(pin: usize) -> Result<(), io::Error> {
    let mut file = try!{File::create("/sys/class/gpio/unexport")};
    try!{write!(file, "{}", pin)};
    Ok(())
}

fn set_out(pin: usize, val: bool) -> Result<(), io::Error> {
    let mut file = try!{File::create(format!("/sys/class/gpio/gpio{}/direction", pin))};
    try!{match val {
        true => write!(file, "out"),
        false => write!(file, "in")
    }};
    Ok(())
}

fn write_val(pin: usize, value: usize) -> Result<(), io::Error> {
    let mut file = try!{File::create(format!("/sys/class/gpio/gpio{}/value", pin))};
    try!{write!(file, "{}", value)};
    Ok(())
}

fn read_val(pin: usize) -> Result<bool, io::Error> {
    let mut file = try!{File::open(format!("/sys/class/gpio/gpio{}/value", pin))};
    let mut buf = { 0 };
    Ok(match try!{file.read(&mut buf)} {
        '0' as u8 => false,
        _ => true
    })
}

fn main() {
    match export(17) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e)
    }
    set_out(17, true).unwrap();
    match export(27) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e)
    }
    set_out(27, false).unwrap();
    loop {
        println!("bing");
        if read_val(27).unwrap() {
            break
        }
        sleep(Duration::from_millis(100));

    }
    write_val(17, 1).unwrap();
    sleep(Duration::from_millis(1000));
    write_val(17, 0).unwrap();
    unexport(17).unwrap();
    unexport(27).unwrap();
}
