use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

enum TriggerType {
    Above,
    Below,
    Equal,
    Charging,
    Discharging,
}

struct Trigger {
    percentage: Option<i32>,
    when: TriggerType,
    message: String,
}

pub struct Config {
    triggers: Vec<Trigger>,
}

#[derive(Debug)]
pub struct ConfigError {
    message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.message)
    }
}

impl Error for ConfigError {}

impl ConfigError {
    fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }
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
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;
        let yaml = YamlLoader::load_from_str(&file_content)?;

        let triggers = Vec::new();
        for entry in &yaml {
            let entry_op = entry.as_hash();
            if let Some(entry) = entry_op {
                let values_yaml = entry.values().next().unwrap();
                let values_op = values_yaml.as_hash();
                if let Some(values) = values_op {
                    let perc: Option<i32>;
                    let when: String;
                    let message: String;

                    if let Some(perc_yaml) = values.get(&Yaml::String(String::from("percentage"))) {
                        if let Some(perc_val) = perc_yaml.as_i64() {
                            perc = Some(perc_val as i32);
                        } else {
                            return Err(Box::new(ConfigError::new("percentage must be integer")));
                        }
                    } else {
                        perc = None;
                    }

                    if let Some(when_yaml) = values.get(&Yaml::String(String::from("when"))) {
                        if let Some(when_val) = when_yaml.as_str() {
                            when = String::from(when_val);
                        } else {
                            return Err(Box::new(ConfigError::new("when field must be a string")));
                        }
                    } else {
                        return Err(Box::new(ConfigError::new("when key missing")));
                    }

                    if let Some(message_yaml) = values.get(&Yaml::String(String::from("message"))) {
                        if let Some(message_val) = message_yaml.as_str() {
                            message = String::from(message_val);
                        } else {
                            return Err(Box::new(ConfigError::new(
                                "message field must be a string",
                            )));
                        }
                    } else {
                        return Err(Box::new(ConfigError::new("meessage key missing")));
                    }
                }
            }
        }
        Ok(Self { triggers })
    }
}
