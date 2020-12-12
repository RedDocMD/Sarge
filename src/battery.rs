use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct BatteryInfo {
    pub charging: bool,
    pub percentage: i32,
    pub charge_now: i32,
    pub charge_full: i32,
}

pub struct InfoDirectories {
    battery: PathBuf,
    ac: PathBuf,
}

impl InfoDirectories {
    pub fn read() -> io::Result<Option<Self>> {
        let power_supply = Path::new("/sys/class/power_supply");
        let mut battery_path = None;
        let mut ac_path = None;
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
            Ok(Some(Self {
                battery: bat,
                ac: ac,
            }))
        } else {
            Ok(None)
        }
    }
}

impl BatteryInfo {
    pub fn from(loc: InfoDirectories) -> Result<Self, Box<dyn Error>> {
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

        Ok(Self {
            charge_now: charge_now.trim().parse()?,
            charge_full: charge_full.trim().parse()?,
            percentage: percentage.trim().parse()?,
            charging: charging == String::from("1"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_dir() -> io::Result<()> {
        let dirs = InfoDirectories::read()?;
        assert_eq!(dirs.is_some(), true);
        if let Some(dirs) = dirs {
            assert_eq!(dirs.battery.is_dir(), true);
            assert_eq!(dirs.ac.is_dir(), true);
        }
        Ok(())
    }

    #[test]
    fn test_info() -> Result<(), Box<dyn Error>> {
        let dirs = InfoDirectories::read()?.unwrap();
        let info = BatteryInfo::from(dirs)?;
        println!("{:?}", info);
        Ok(())
    }
}
