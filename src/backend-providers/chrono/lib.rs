#[macro_use]
extern crate log;

pub use config::*;

mod config;
mod time_provider_impl;

pub struct Provider {
    config: Config,
}

impl Provider {
    pub fn new(config: Config) -> Self {
        info!("Initializing Chrono provider with configuration: {:?}", config);

        Self { config }
    }
}