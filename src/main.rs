extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let pin = Pin::new(17);
    pin.with_exported(|| {
        loop {
            pin.set_value(1).unwrap();
            sleep(Duration::from_millis(200));
            pin.set_value(0).unwrap();
            sleep(Duration::from_millis(200));
        }
    }).unwrap();
}
