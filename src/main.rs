extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let pin = Pin::new(17);
    pin.export().unwrap();
    pin.set_value(1).unwrap();
    sleep(Duration::from_millis(500));
    pin.set_value(0).unwrap();
    pin.unexport().unwrap();

}
