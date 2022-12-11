mod actor;
mod ingics;
mod sensor;
mod settings;
mod weather;

use settings::Settings;

use chrono::Utc;
use std::thread;
use std::time;
//use chrono_tz::Tz;
use log::{debug, info};

fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    version.to_string()
}

pub fn run(config_file: String) {
    // setup settings config
    let settings = match Settings::new(config_file) {
        Ok(settings) => settings,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    debug!("{:?}", settings.general);
    debug!("{:?}", settings.sensors);
    debug!("{:?}", settings.actors);

    info!(
        "Welcome to rs-terrarium {}. Loaded {} Sensors and {} Actors",
        get_version(),
        settings.sensors.len(),
        settings.actors.len()
    );

    thread::spawn(ingics::run);

    let mut actors = actor::init(settings.actors);
    //for actor in &actors {
    //    actor.print_actor_config();
    //}

    // start our weather observatory via OWM
    let receiver = &openweathermap::init(
        settings.general.weather.location.as_str(),
        settings.general.weather.units.as_str(),
        settings.general.weather.language.as_str(),
        settings.general.weather.api_key.as_str(),
        60, // minutes
    );

    // I'm assuming that we don't want to enable the actors until we have a new suntime from the
    // API. That's why I set the initial values to some hours in the future.
    // Not sure if that ever was necessary, but I'm leaving it here for now.
    let mut suntime = weather::Suntime {
        sunrise: Utc::now() + chrono::Duration::hours(24),
        sunset: Utc::now() + chrono::Duration::hours(24),
        updated: false,
    };

    while !suntime.updated {
        weather::update_suntime(receiver, &mut suntime);
    }

    loop {
        weather::update_suntime(receiver, &mut suntime);

        actors.iter_mut().for_each(|actor| {
            //let on_delay = iso8601_duration::Duration::parse(actor.config.on_delay);

            actor.toggle_timebased(suntime.sunrise, suntime.sunset);
        });

        thread::sleep(time::Duration::from_millis(100));
    }
}
