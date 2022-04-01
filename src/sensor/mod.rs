//pub mod bme280;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Model {
    BME280,
    HCSR04,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct SensorConfig {
    name: String,
    model: Model,
    address: Option<String>,
    path: Option<String>,
}

//trait Sensor {
//    fn new(name: String, address: String, model: Model) -> Self;
//    fn read(&self);
//}
