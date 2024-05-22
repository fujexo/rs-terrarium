use super::settings::Weather;
use chrono::{DateTime, Datelike, TimeZone, Utc};
use log::{error, info};
use sunrise_sunset_calculator::SunriseSunsetParameters;

pub struct Suntime {
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    pub initialized: bool,
}

pub fn update_suntime(location: &Weather, suntime: &mut Suntime) {
    let today_date = Utc::now().date_naive();
    let today_timestamp =
        chrono::NaiveDate::from_ymd_opt(today_date.year(), today_date.month(), today_date.day())
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp();
    let result =
        SunriseSunsetParameters::new(today_timestamp, location.latitude, location.longitude)
            .calculate();

    match result {
        Ok(data) => {
            let sunrise = Utc.timestamp_opt(data.rise, 0).unwrap();
            let sunset = Utc.timestamp_opt(data.set, 0).unwrap();

            if (suntime.sunrise != sunrise || suntime.sunset != sunset)
                && (sunrise.date_naive() == today_date || !suntime.initialized)
            {
                let ontime = sunset.signed_duration_since(sunrise);
                let on_hours = ontime.num_hours();
                let on_minutes = ontime.num_minutes() - on_hours * 60;

                info!(
                    "Sunrise/sunset changed: Sunrise {}, Sunset {}, Ontime {}h {}m",
                    sunrise, sunset, on_hours, on_minutes
                );
                suntime.sunrise = sunrise;
                suntime.sunset = sunset;
                suntime.initialized = true;
            }
        }
        Err(e) => {
            error!("Failed to get suntime: {}", e);
        }
    }
}
