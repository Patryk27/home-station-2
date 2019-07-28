use lib_app_config::Config;
use lib_app_config::providers::TimeProviderName;
use lib_backend_common::TimeProvider;
use lib_backend_providers::{chrono, dummy};

pub fn init_time_provider(config: &Config) -> Box<dyn TimeProvider> {
    match config.providers.time {
        TimeProviderName::Chrono => {
            let config = config
                .providers
                .settings
                .chrono.as_ref().expect("Chrono provider has not been configured - please create the [providers.settings.chrono] section.")
                .clone();

            Box::new(chrono::Provider::new(config))
        }

        TimeProviderName::Dummy => {
            Box::new(dummy::Provider::new())
        }
    }
}