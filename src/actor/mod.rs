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

    fn print_actor_config(&self) {}
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

fn parse_delay(seconds: &String) -> Result<chrono::Duration, String> {
    let chrono_duration = chrono::Duration::seconds(seconds.parse::<i64>().unwrap());

    Ok(chrono_duration)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_actor_init() {
        let actor_configs = vec![
            ActorConfig {
                name: "test_actor".to_string(),
                address: "test_actor".to_string(),
                kind: "dummy".to_string(),
                max_ontime: Some("10:00".to_string()),
                min_ontime: Some("08:00".to_string()),
                on_delay: Some("10".to_string()),
                off_delay: Some("10".to_string()),
            },
            ActorConfig {
                name: "test_actor2".to_string(),
                address: "test_actor2".to_string(),
                kind: "dummy".to_string(),
                max_ontime: None,
                min_ontime: None,
                on_delay: None,
                off_delay: None,
            },
        ];

        let actors = init(actor_configs);

        assert_eq!(actors.len(), 2);
        assert_eq!(actors[0].get_state(), State::Off);
    }
}
