use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Weather {
    pub api_key: String,
    pub location: String,
    pub units: String,
    pub language: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct General {
    pub timezone: String,
    pub weather: Weather,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Settings {
    pub general: General,
    pub sensors: Vec<crate::sensor::SensorConfig>,
    pub actors: Vec<crate::actor::ActorConfig>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("/etc/terrarium/config.toml"))
            .build()?;

        config.try_deserialize()
    }
}
