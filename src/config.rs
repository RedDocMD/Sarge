use std::error::Error;
use std::path::Path;

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

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {}
}
