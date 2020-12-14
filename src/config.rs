use super::battery::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
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
        let mut config: Config = serde_yaml::from_str(&content)?;
        for trigger in &mut config.triggers {
            match trigger.when {
                TriggerType::Charging | TriggerType::Discharging => trigger.percentage = None,
                _ => {}
            };
        }
        Ok(config)
    }

    pub fn messages(&self, old: &BatteryInfo, new: &BatteryInfo) -> Vec<String> {
        let mut msgs = Vec::new();
        for trigger in &self.triggers {
            match trigger.when {
                TriggerType::Above => {
                    if old.percentage <= trigger.percentage.unwrap()
                        && new.percentage > trigger.percentage.unwrap()
                    {
                        msgs.push(trigger.message.clone());
                    }
                }
                TriggerType::Below => {
                    if old.percentage >= trigger.percentage.unwrap()
                        && new.percentage < trigger.percentage.unwrap()
                    {
                        msgs.push(trigger.message.clone());
                    }
                }
                TriggerType::Equal => {
                    if old.percentage != trigger.percentage.unwrap()
                        && new.percentage == trigger.percentage.unwrap()
                    {
                        msgs.push(trigger.message.clone());
                    }
                }
                TriggerType::Charging => {
                    if !old.charging && new.charging {
                        msgs.push(trigger.message.clone());
                    }
                }
                TriggerType::Discharging => {
                    if old.charging && !new.charging {
                        msgs.push(trigger.message.clone());
                    }
                }
            };
        }
        msgs
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn simple_conf() -> Result<(), Box<dyn Error>> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_data/confs/simple.yaml");
        let config = Config::from_file(&path)?;

        let expected_triggers = vec![
            Trigger {
                percentage: Some(20),
                when: TriggerType::Below,
                message: String::from("Battery low"),
            },
            Trigger {
                percentage: Some(100),
                when: TriggerType::Equal,
                message: String::from("Fully charged"),
            },
            Trigger {
                percentage: None,
                when: TriggerType::Discharging,
                message: String::from("Battery discharging"),
            },
        ];

        assert_eq!(expected_triggers, config.triggers);
        Ok(())
    }

    #[test]
    fn auto_none_conf() -> Result<(), Box<dyn Error>> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_data/confs/auto_none.yaml");
        let config = Config::from_file(&path)?;

        let expected_triggers = vec![Trigger {
            percentage: None,
            when: TriggerType::Charging,
            message: String::from("Battery charging"),
        }];

        assert_eq!(expected_triggers, config.triggers);
        Ok(())
    }

    #[test]
    fn garbage_value_conf() -> Result<(), Box<dyn Error>> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_data/confs/garbage_value.yaml");
        assert!(
            Config::from_file(&path).is_err(),
            "shouldn't accept wrong key for when"
        );
        Ok(())
    }
}
