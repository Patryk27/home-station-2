#[macro_use]
extern crate log;

mod time_provider_impl;
mod weather_provider_impl;

pub struct Provider;

impl Provider {
    pub fn new() -> Self {
        info!("Initializing dummy provider");

        Self
    }
}