use log::{error, warn, LevelFilter};
use sarge::battery::*;
use sarge::config::*;
use std::env;
use std::path::PathBuf;
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

    let mut config: Config;
    let mut conf_set = false;
    if let Some(xdg_config_home) = env::var_os("XDG_CONFIG_HOME") {
        let mut path = PathBuf::from(xdg_config_home);
        path.push("sarge");
        path.push("sargee.yml");
        if path.exists() {
            if let Ok(conf) = Config::from_file(&path) {
                config = conf;
                conf_set = true;
            }
        } else {
            path.pop();
            path.pop();
            path.push("sarge.yml");
            if path.exists() {
                if let Ok(conf) = Config::from_file(&path) {
                    config = conf;
                    conf_set = true;
                }
            }
        }
    }
    if !conf_set {
        if let Some(home) = env::var_os("HOME") {
            let mut path = PathBuf::from(home);
            path.push(".config");
            path.push("sarge");
            path.push("sarge.yml");
            if path.exists() {
                if let Ok(conf) = Config::from_file(&path) {
                    config = conf;
                    conf_set = true;
                }
            } else {
                path.pop();
                path.pop();
                path.pop();
                path.push(".sarge.yml");
                if path.exists() {
                    if let Ok(conf) = Config::from_file(&path) {
                        config = conf;
                        conf_set = true;
                    }
                }
            }
        }
    }
    if !conf_set {
        config = Config::default();
        warn!("No explicit config file found; falling back to default config");
    }
}
