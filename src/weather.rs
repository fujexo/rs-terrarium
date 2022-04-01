use chrono::{DateTime, TimeZone, Utc};
use log::{error, info};

pub struct Suntime {
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    pub updated: bool,
}

pub fn update_suntime(receiver: &openweathermap::Receiver, suntime: &mut Suntime) {
    match openweathermap::update(receiver) {
        Some(response) => match response {
            Ok(data) => {
                let sunrise = Utc.timestamp(data.sys.sunrise, 0);
                let sunset = Utc.timestamp(data.sys.sunset, 0);

                if suntime.sunrise != sunrise || suntime.sunset != sunset {
                    let ontime = sunset.signed_duration_since(sunrise);
                    let on_hours = ontime.num_hours();
                    let on_minutes = ontime.num_minutes() - on_hours * 60;

                    info!(
                        "Sunrise/sunset changed: Sunrise {}, Sunset {}, Ontime {}h {}m",
                        sunrise, sunset, on_hours, on_minutes
                    );
                    suntime.sunrise = sunrise;
                    suntime.sunset = sunset;
                    suntime.updated = true;
                }
            }
            Err(e) => {
                if e == "loading..." {
                    // Loading should happen only at the first call and is ignored
                } else {
                    error!("Failed to get suntime: {}", e);
                }
            }
        },
        None => (),
    }
}
