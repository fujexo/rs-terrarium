use super::{Actor, ActorConfig, State};
use chrono::{DateTime, Utc};
use log::{debug, error, info};

#[derive(Debug)]
pub struct Dummy {
    config: ActorConfig,
    state: State,
}

impl Actor for Dummy {
    fn new(actor_config: ActorConfig) -> Self {
        Dummy {
            config: actor_config,
            state: State::Off,
        }
    }

    fn get_state(&self) -> State {
        match self.state {
            State::On => State::On,
            State::Off => State::Off,
        }
    }

    fn on(&mut self) {
        self.state = State::On;
        info!("Actor '{}' is now on", self.config.name);
    }

    fn off(&mut self) {
        self.state = State::Off;
        info!("Actor '{}' is now off", self.config.name);
    }

    fn print_actor_config(&self) {
        dbg!(&self.config);
    }

    // Toggle the actor based on a start time and end time
    fn toggle_timebased(&mut self, start: DateTime<Utc>, end: DateTime<Utc>) {
        let now = Utc::now();

        let on_delay = super::parse_delay(&self.config, "on_delay");
        let off_delay = super::parse_delay(&self.config, "off_delay");

        let start = match on_delay {
            Ok(delay) => start + delay,
            Err(e) => {
                error!("Error parsing on_delay: {}", e);
                start
            }
        };

        let end = match off_delay {
            Ok(delay) => end + delay,
            Err(e) => {
                error!("Error parsing off_delay: {}", e);
                end
            }
        };

        match self.get_state() {
            State::Off => {
                if now > start && now < end {
                    debug!("Start: {:?}, End: {:?}", start, end);
                    self.on();
                }
            }
            State::On => {
                if now > end || now < start {
                    debug!("Start: {:?}, End: {:?}", start, end);
                    self.off();
                }
            }
        }
    }
}
