use crate::configuration::model::AppConfig;
use std::error::Error;

pub fn load_config(path: &str) -> Result<AppConfig, Box<dyn Error>> {
    let f = std::fs::File::open(path)?;
    //.expect("open config yaml file error");

    let app_config: AppConfig = serde_yaml::from_reader(f)?;
    //.expect("read value from file error");

    return Ok(app_config);
}
