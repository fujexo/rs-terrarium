pub mod dummy;
pub mod gpio_relay;

use chrono::{DateTime, Utc};
use log::debug;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct ActorConfig {
    name: String,
    address: String,
    kind: String,
    max_ontime: Option<String>,
    min_ontime: Option<String>,
    on_delay: Option<String>,
    off_delay: Option<String>,
}

#[derive(Debug)]
pub enum State {
    On,
    Off,
}

pub trait Actor {
    // Create a new actor based on the type
    fn new(actor_config: ActorConfig) -> Self
    where
        Self: Sized;

    // Turn the actor on
    fn on(&mut self) {}

    // Turn the actor off
    fn off(&mut self) {}

    // Get the current state of the actor
    fn get_state(&self) -> State;

    // Toggle the state of the actor
    fn toggle(&mut self) {
        match self.get_state() {
            State::On => self.off(),
            State::Off => self.on(),
        }
    }

    // Toggle the actor based on a start time and end time
    fn toggle_timebased(&mut self, start: DateTime<Utc>, end: DateTime<Utc>) {
        let now = Utc::now();

        match self.get_state() {
            State::Off => {
                if now > start && now < end {
                    self.on();
                }
            }
            State::On => {
                if now > end || now < start {
                    self.off();
                }
            }
        }
    }
}

pub fn init(actor_configs: Vec<ActorConfig>) -> Vec<Box<dyn Actor>> {
    let mut actors: Vec<Box<dyn Actor>> = Vec::new();

    for actor in actor_configs {
        match actor.kind.as_str() {
            "gpio_relay" => {
                debug!("Adding gpio_relay actor: {:?}", actor);
                let actor = gpio_relay::GpioRelay::new(actor);
                actors.push(Box::new(actor));
            }
            _ => {
                debug!("Adding dummy actor: {:?}", actor);
                let actor = dummy::Dummy::new(actor);
                actors.push(Box::new(actor));
            }
        }
    }

    actors
}
