pub mod dummy;
pub mod gpio_relay;

#[cfg(test)]
use mockall::{automock, mock, predicate::*};

use chrono::{DateTime, Utc};
use log::debug;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct ActorConfig {
    name: String,
    address: String,
    kind: String,
    max_ontime: Option<i64>,
    min_ontime: Option<i64>,
    on_delay: Option<i64>,
    off_delay: Option<i64>,
}

#[derive(Debug)]
pub enum State {
    On,
    Off,
}

#[cfg_attr(test, automock)]
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
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

fn parse_duration_seconds(duration: Option<&i64>) -> Result<chrono::Duration, String> {
    match duration {
        Some(duration) => Ok(chrono::Duration::seconds(*duration)),
        None => Err("No duration specified".to_string()),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::Duration;

    fn test_init() -> Vec<ActorConfig> {
        vec![
            ActorConfig {
                name: "test_actor".to_string(),
                address: "test_actor".to_string(),
                kind: "dummy".to_string(),
                max_ontime: Some(72000),
                min_ontime: Some(3600),
                on_delay: Some(600),
                off_delay: Some(-600),
            },
            ActorConfig {
                name: "test_actor2".to_string(),
                address: "test_actor2".to_string(),
                kind: "dummy".to_string(),
                max_ontime: None,
                min_ontime: Some(3600),
                on_delay: None,
                off_delay: None,
            },
        ]
    }

    #[test]
    fn actor_init() {
        let actors_config = test_init();
        let actors = init(actors_config);

        assert_eq!(actors.len(), 2);
        matches!(actors[0].get_state(), State::Off);
        matches!(actors[1].get_state(), State::Off);
    }

    #[test]
    fn test_parse_duration_seconds() {
        let duration = parse_duration_seconds(Some(&10));
        assert_eq!(duration.unwrap().num_seconds(), 10);

        let duration = parse_duration_seconds(None);
        matches!(duration, Err(_));
    }

    #[test]
    fn test_actor_on_off() {
        let actors_config = test_init();
        let mut actors = init(actors_config);

        matches!(actors[0].get_state(), State::Off);
        actors[0].on();
        matches!(actors[0].get_state(), State::On);
        actors[0].off();
        matches!(actors[0].get_state(), State::Off);
    }

    #[test]
    fn test_actor_toggle() {
        let actors_config = test_init();
        let mut actors = init(actors_config);

        matches!(actors[0].get_state(), State::Off);
        actors[0].toggle();
        matches!(actors[0].get_state(), State::On);
        actors[0].toggle();
        matches!(actors[0].get_state(), State::Off);
    }

    #[test]
    fn test_actor_toggle_timebased() {
        struct Test {
            actor_config: ActorConfig,
            state: State,
        }

        impl Actor for Test {
            fn new(actor_config: ActorConfig) -> Self {
                Test {
                    state: State::Off,
                    actor_config,
                }
            }
            fn on(&mut self) {
                self.state = State::On;
            }
            fn off(&mut self) {
                self.state = State::Off;
            }
            fn get_state(self: &Test) -> State {
                match self.state {
                    State::On => State::On,
                    State::Off => State::Off,
                }
            }
        }

        let mut test_actor = Test::new(ActorConfig {
            name: "Test Actor Name".to_string(),
            address: "Test address".to_string(),
            kind: "Test Actor kind".to_string(),
            on_delay: None,
            off_delay: None,
            max_ontime: None,
            min_ontime: None,
        });

        let now = Utc::now();

        test_actor.toggle_timebased(now - Duration::hours(1), now + Duration::hours(1));
        matches!(test_actor.get_state(), State::On);

        test_actor.toggle_timebased(now + Duration::hours(1), now + Duration::hours(2));
        matches!(test_actor.get_state(), State::Off);

        test_actor.toggle_timebased(now - Duration::hours(2), now - Duration::hours(1));
        matches!(test_actor.get_state(), State::Off);
    }

    /*Not sure how to test this with mockall... TODO revisit in the future*/
    //#[test]
    //fn test_on_off() {
    //    let mut mock = MockActor::new(ActorConfig {
    //        name: "Test Actor Name".to_string(),
    //        address: "Test address".to_string(),
    //        kind: "Test Actor kind".to_string(),
    //        on_delay: None,
    //        off_delay: None,
    //        max_ontime: None,
    //        min_ontime: None,
    //    });

    //    mock.expect_on().times(1).return_const(());
    //    mock.expect_off().times(1).return_const(());

    //    assert_eq!((), mock.on());
    //    assert_eq!((), mock.off());
    //}
}
