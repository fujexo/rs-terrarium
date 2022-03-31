pub struct BME280Config {
    i2c_address: (String, String),
    i2c_id: f32,
}

pub struct BME280Result {
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
}

impl BME280Config {
    pub fn new(i2c_address: (String, String), i2c_id: f32) -> Self {
        Self {
            i2c_address,
            i2c_id,
        }
    }

    pub fn read_sensor(&self) -> Result<BME280Result, String> {
        //TODO Error handling
        Ok(BME280Result {
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0.0,
        })
    }
}
