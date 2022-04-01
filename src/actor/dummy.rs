use super::{Actor, ActorConfig, State};
use log::info;

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
}
