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
pub struct InfluxDB {
    pub hostname: String,
    pub port: isize,
    pub database: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Settings {
    pub general: General,
    pub influxdb: InfluxDB,
    pub sensors: Vec<crate::sensor::SensorConfig>,
    pub actors: Vec<crate::actor::ActorConfig>,
}

impl Settings {
    pub fn new(config_file: String) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .set_default("influxdb.hostname", "localhost")?
            .set_default("influxdb.port", 8089)?
            .set_default("influxdb.database", "db0")?
            .add_source(File::with_name(&config_file))
            .build()?;

        config.try_deserialize()
    }
}
