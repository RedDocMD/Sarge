use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct BatteryInfo {
    charging: bool,
    percentage: f32,
    charge_now: i32,
    charge_full: i32,
}

pub struct InfoDirectories {
    battery: PathBuf,
    ac: PathBuf,
}

pub fn info_directories() -> io::Result<Option<InfoDirectories>> {
    let power_supply = Path::new("/sys/class/power_supply");
    let mut battery_path: Option<PathBuf> = None;
    let mut ac_path: Option<PathBuf> = None;
    for entry in fs::read_dir(power_supply)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let name = path.to_str().unwrap();
            if name.contains("BAT") {
                battery_path = Some(path);
            } else if name.contains("AC") {
                ac_path = Some(path);
            }
        }
    }
    if let (Some(bat), Some(ac)) = (battery_path, ac_path) {
        Ok(Some(InfoDirectories {
            battery: bat,
            ac: ac,
        }))
    } else {
        Ok(None)
    }
}

pub fn battery_info(loc: InfoDirectories) -> io::Result<BatteryInfo> {
    let charge_now_path = loc.battery.join(Path::new("charge_now"));
    let charge_full_path = loc.battery.join(Path::new("charge_full"));
    let percentage_path = loc.battery.join(Path::new("capacity"));
    let charging_path = loc.ac.join(Path::new("online"));

    let mut charge_now = String::new();
    let mut charge_full = String::new();
    let mut percentage = String::new();
    let mut charging = String::new();

    File::open(charge_now_path)?.read_to_string(&mut charge_now)?;
    File::open(charge_full_path)?.read_to_string(&mut charge_full)?;
    File::open(percentage_path)?.read_to_string(&mut percentage)?;
    File::open(charging_path)?.read_to_string(&mut charging)?;

    let is_charging = charging == String::from("1");

    Ok(BatteryInfo {
        charge_now: charge_now.parse().unwrap(),
        charge_full: charge_full.parse().unwrap(),
        percentage: percentage.parse().unwrap(),
        charging: is_charging,
    })
}
