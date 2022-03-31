use log::info;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct ActorConfig {
    name: String,
    address: String,
    max_ontime: Option<String>,
    min_ontime: Option<String>,
    on_delay: Option<String>,
    off_delay: Option<String>,
}

#[derive(Serialize, Debug)]
pub enum State {
    On,
    Off,
}

#[derive(Serialize, Debug)]
pub struct Actor {
    config: ActorConfig,
    state: State,
}

impl Actor {
    pub fn new(actor_config: ActorConfig) -> Self {
        Actor {
            config: actor_config,
            state: State::Off,
        }
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn on(&mut self) {
        self.state = State::On;
        info!("Actor '{}' is now on", self.config.name);
    }

    pub fn off(&mut self) {
        self.state = State::Off;
        info!("Actor '{}' is now off", self.config.name);
    }

    pub fn toggle(&mut self) {
        match self.state {
            State::On => self.off(),
            State::Off => self.on(),
        }
    }
}
