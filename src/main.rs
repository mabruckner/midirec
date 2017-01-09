extern crate rustc_serialize;
extern crate toml;
extern crate libc;
extern crate time;

use std::thread::sleep;
use std::time::Duration;
use std::env::args;

use std::process::Stdio;
use std::process::Command;
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
    let mut buf = [ 0x00 ];
    let val = try!{file.read(&mut buf)};
    match val {
        1 => Ok(buf[0] != '0' as u8),
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "could not read"))
    }
}


#[derive(Debug, RustcDecodable)]
struct Config {
    output_dir: String,
    midi_port: String,
    trigger_pin: usize,
    light_pin: usize,
}

fn wait_til_lift(trigger: usize) {
    while read_val(trigger).unwrap() {
        sleep(Duration::from_millis(50));
    }
}

fn record(trigger: usize, port: &str, dir: &str) {
    let mut child = Command::new("arecordmidi")
        .arg("-p")
        .arg(port)
        .arg(format!("{}/{}.mid", dir, time::get_time().sec))
        .stdin(Stdio::piped())
        .spawn().unwrap();
    while !read_val(trigger).unwrap() {
        sleep(Duration::from_millis(100));
    }
    wait_til_lift(trigger);
    unsafe{ libc::kill(child.id() as i32, libc::SIGINT) };
    child.wait().unwrap();
}

fn main() {
    let mut config_string = String::new();
    File::open(args().nth(1).unwrap()).unwrap().read_to_string(&mut config_string).unwrap();
    let config: Config = toml::decode_str(&config_string).unwrap();
    match export(config.trigger_pin) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e)
    }
    set_out(config.trigger_pin, false).unwrap();
    match export(config.light_pin) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e)
    }
    set_out(config.light_pin, true).unwrap();

    loop {
        if read_val(config.trigger_pin).unwrap() {
            wait_til_lift(config.trigger_pin);
            write_val(config.light_pin, 1).unwrap();
            record(config.trigger_pin, &config.midi_port, &config.output_dir);
            write_val(config.light_pin, 0).unwrap();
        }
        sleep(Duration::from_millis(100));
    }
}
