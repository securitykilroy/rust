use log::{info, error, warn};
use winlog;

fn main() {
    
    winlog::register("WasHere Log");
    winlog::init("WasHere Log").unwrap();
    warn!("This is a log message");
    error!("This is a serious error message");
    info!("Just some information");

}