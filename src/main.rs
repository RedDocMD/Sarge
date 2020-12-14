use log::{error, warn, LevelFilter};
use sarge::battery::*;
use sarge::config::*;
use std::env;
use std::process;
use syslog::{BasicLogger, Facility, Formatter3164};

fn main() {
    if env::consts::OS != "linux" {
        println!("Only supported on Linux");
        process::exit(1);
    }

    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "sarge".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Warn))
        .expect("logger setup error");

    let info_dir = match InfoDirectories::read() {
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
        Ok(s) => match s {
            None => {
                error!("/sys/class/power_supply/BAT* or /sys/class/power_supply/ACAD not found");
                process::exit(1);
            }
            Some(s) => s,
        },
    };
}
