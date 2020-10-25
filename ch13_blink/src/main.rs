use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use syslog::{Formatter3164, Facility};
use sysinfo::get_current_pid;

fn get_process_name() -> String {
    let this_process = std::env::current_exe().unwrap();
    let this_file = this_process.file_name().unwrap();

    String::from(this_file.to_str().unwrap())
}

fn logger(message: &str) {
    let this_pid = get_current_pid().unwrap();
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: get_process_name(),
        pid: this_pid,
    };
    match syslog::unix(formatter) {
       Err(e) => println!("Unable to connect to syslog: {:?}", e),
       Ok(mut writer) => {
           writer.err(message).expect("could not write error message");
       }
    }
}


fn main() {
    let mut rand_instance = rand::thread_rng();

    let my_led = Pin::new(17); 
    my_led.with_exported(|| {
        my_led.set_direction(Direction::Out).unwrap();
        loop {
            my_led.set_value(0).unwrap();
            let length: u64 = rand_instance.gen_range(150,500);
			logger(format!("Waiting {} milliseconds before resetting the light", length).as_str());
            sleep(Duration::from_millis(length));
            my_led.set_value(1).unwrap();
            sleep(Duration::from_millis(length));
        }
    }).unwrap();
}
