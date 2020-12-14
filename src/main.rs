use log::{error, warn, LevelFilter};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use notify_rust::Notification;
use sarge::battery::*;
use sarge::config::*;
use std::env;
use std::path::PathBuf;
use std::process;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
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

    let (config_raw, config_path) = get_config();
    let config_rc = Arc::from(Mutex::new(config_raw));

    if let Some(config_path) = config_path {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(5)).unwrap();
        watcher
            .watch(config_path, RecursiveMode::NonRecursive)
            .unwrap();

        let config_rc = Arc::clone(&config_rc);
        thread::spawn(move || loop {
            let event = rx.recv().unwrap();
            match event {
                DebouncedEvent::Write(path) => {
                    if let Ok(conf) = &mut Config::from_file(&path) {
                        let mut config = config_rc.lock().unwrap();
                        println!("Got hands on config: {}", path.display());
                        config.update(conf);
                    }
                }
                _ => {}
            };
        });
    } else {
        warn!("no config file found; choosing to default config");
    }

    let mut old_info = get_info(&info_dir);
    let config = config_rc.lock().unwrap();
    thread::sleep(config.intv());
    drop(config);
    let mut new_info = get_info(&info_dir);

    loop {
        let config = config_rc.lock().unwrap();
        let msgs = config.messages(&old_info, &new_info);
        if msgs.len() != 0 {
            for msg in &msgs {
                match Notification::new().summary("sarge").body(msg).show() {
                    Err(e) => error!("{}", e),
                    _ => {}
                };
            }
        }
        thread::sleep(config.intv());
        drop(config);
        old_info = new_info;
        new_info = get_info(&info_dir);
    }
}

fn get_config() -> (Config, Option<PathBuf>) {
    if let Some(xdg_config_home) = env::var_os("XDG_CONFIG_HOME") {
        let mut path = PathBuf::from(xdg_config_home);
        path.push("sarge");
        path.push("sarge.yml");
        if path.exists() {
            match Config::from_file(&path) {
                Ok(c) => return (c, Some(path)),
                Err(e) => warn!("error in {}: {}", path.display(), e),
            };
        } else {
            path.pop();
            path.pop();
            path.push("sarge.yml");
            if path.exists() {
                match Config::from_file(&path) {
                    Ok(c) => return (c, Some(path)),
                    Err(e) => warn!("error in {}: {}", path.display(), e),
                };
            }
        }
    }
    if let Some(home) = env::var_os("HOME") {
        let mut path = PathBuf::from(home);
        path.push(".config");
        path.push("sarge");
        path.push("sarge.yml");
        if path.exists() {
            match Config::from_file(&path) {
                Ok(c) => return (c, Some(path)),
                Err(e) => warn!("error in {}: {}", path.display(), e),
            };
        } else {
            path.pop();
            path.pop();
            path.pop();
            path.push(".sarge.yml");
            if path.exists() {
                match Config::from_file(&path) {
                    Ok(c) => return (c, Some(path)),
                    Err(e) => warn!("error in {}: {}", path.display(), e),
                };
            }
        }
    }
    (Config::default(), None)
}

fn get_info(info_dir: &InfoDirectories) -> BatteryInfo {
    match BatteryInfo::from(info_dir) {
        Ok(s) => s,
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    }
}
