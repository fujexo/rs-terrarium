use super::{Actor, ActorConfig, State};
use log::info;
use rppal::gpio::{Gpio, OutputPin};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct GpioRelay {
    config: ActorConfig,
    pin: OutputPin,
}

impl Actor for GpioRelay {
    fn new(actor_config: ActorConfig) -> Self {
        let pin = match Gpio::new() {
            Ok(gpio) => gpio,
            Err(e) => {
                panic!("Error initializing GPIO: {}", e);
            }
        };

        let pin = match pin.get(actor_config.address.parse::<u8>().unwrap()) {
            Ok(pin) => pin,
            Err(e) => {
                panic!("Error getting pin {}: {}", actor_config.address, e);
            }
        };

        let pin = pin.into_output();

        //let pin = Gpio::new()
        //    .unwrap()
        //    .get(actor_config.address.parse::<u8>().unwrap())
        //    .unwrap()
        //    .into_output();
        GpioRelay {
            config: actor_config,
            pin: pin,
        }
    }
    fn get_state(&self) -> State {
        if self.pin.is_set_high() {
            State::On
        } else {
            State::Off
        }
    }

    fn on(&mut self) {
        self.pin.set_high();
        thread::sleep(Duration::from_millis(500));
        info!("Actor '{}' is now on", self.config.name);
    }

    fn off(&mut self) {
        self.pin.set_low();
        thread::sleep(Duration::from_millis(500));
        info!("Actor '{}' is now off", self.config.name);
    }
}
