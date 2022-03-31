mod actor;
mod sensor;
mod settings;
mod weather;

use actor::Actor;
use settings::Settings;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use log::debug;

pub fn init_actors(actors: Vec<actor::ActorConfig>) -> Vec<actor::Actor> {
    actors
        .into_iter()
        .map(|config| actor::Actor::new(config))
        .collect()
}

pub fn check_relay_state(actor: &mut actor::Actor, sunrise: DateTime<Utc>, sunset: DateTime<Utc>) {
    let now = Utc::now();

    match actor.get_state() {
        actor::State::Off => {
            if now > sunrise && now < sunset {
                actor.on();
            }
        }
        actor::State::On => {
            if now > sunset || now < sunrise {
                actor.off();
            }
        }
    }
}

pub fn run() -> () {
    // setup settings config
    let settings = match Settings::new() {
        Ok(settings) => settings,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    debug!("{:?}", settings.general);
    debug!("{:?}", settings.sensors);
    debug!("{:?}", settings.actors);

    let usertimezone: Tz = match settings.general.timezone.parse() {
        Ok(tz) => tz,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    //let mut actors = terrarium::init_actors(settings.actors);
    //let mut relay01 = actors.get(0).unwrap();
    let mut relay01 = Actor::new(settings.actors[0].clone());

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

    //let testtime = Suntime {
    //    sunrise: Utc::now() + chrono::Duration::minutes(1),
    //    sunset: Utc::now() + chrono::Duration::minutes(2),
    //    updated: true,
    //};
    //info!("Sunrise: {}", sunrise.with_timezone(&usertimezone));
    //info!("Sunset: {}", sunset.with_timezone(&usertimezone));

    while suntime.updated == false {
        weather::update_suntime(receiver, &mut suntime);
    }

    loop {
        weather::update_suntime(receiver, &mut suntime);

        check_relay_state(&mut relay01, suntime.sunrise, suntime.sunset);
    }
}
