use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main()  -> sysfs_gpio::Result<()> {
    let inpin = Pin::new(23);

    inpin.with_exported(|| {
        inpin.set_direction(Direction::In)?;
        let mut prev_val: u8 = 255;
        loop {
            let val = inpin.get_value()?;
            if val != prev_val {
				println!("Button has been pushed with a value of {}", val);
                prev_val = val;
            }
            sleep(Duration::from_millis(10));
        }
    })
}

