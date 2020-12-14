use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum TriggerType {
    Above,
    Below,
    Equal,
    Charging,
    Discharging,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Trigger {
    percentage: Option<i32>,
    when: TriggerType,
    message: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    triggers: Vec<Trigger>,
}

impl Config {
    pub fn default() -> Self {
        let triggers = vec![
            Trigger {
                percentage: Some(100),
                when: TriggerType::Equal,
                message: String::from("Battery full"),
            },
            Trigger {
                percentage: Some(20),
                when: TriggerType::Below,
                message: String::from("Battery low - less than 20% remaining"),
            },
            Trigger {
                percentage: None,
                when: TriggerType::Charging,
                message: String::from("Battery charging"),
            },
            Trigger {
                percentage: None,
                when: TriggerType::Discharging,
                message: String::from("Battery discharging"),
            },
        ];
        Self { triggers }
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_conf() {}
}
