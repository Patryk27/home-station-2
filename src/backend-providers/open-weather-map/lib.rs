#[macro_use]
extern crate log;

pub use config::Config;

mod config;
mod weather_provider_impl;

pub struct Provider {
    pub(crate) config: Config,
}

impl Provider {
    pub fn new(config: Config) -> Self {
        info!("Initializing OpenWeatherMap provider with configuration: {:?}", config);

        Self { config }
    }
}