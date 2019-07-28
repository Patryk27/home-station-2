#[macro_use]
extern crate log;

use reqwest::Client;

pub use config::Config;

mod config;
mod weather_provider_impl;

pub struct Provider {
    pub(crate) client: Client,
    pub(crate) config: Config,
}

impl Provider {
    pub fn new(config: Config) -> Self {
        info!("Initializing DarkSky provider with configuration: {:?}", config);

        Self { client: Client::new(), config }
    }
}