use sarge::config::*;

fn main() -> Result<(), serde_yaml::Error> {
    let defconf = Config::default();
    let yaml = serde_yaml::to_string(&defconf)?;
    println!("{}", yaml);
    Ok(())
}
