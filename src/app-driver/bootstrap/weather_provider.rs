use lib_app_config::Config;
use lib_app_config::providers::WeatherProviderName;
use lib_backend_common::WeatherProvider;
use lib_backend_providers::{dark_sky, dummy, open_weather_map};

pub fn init_weather_provider(config: &Config) -> Box<dyn WeatherProvider> {
    match config.providers.weather {
        WeatherProviderName::DarkSky => {
            let config = config
                .providers
                .settings
                .dark_sky.as_ref().expect("DarkSky provider has not been configured - please create the [providers.settings.dark-sky] section.")
                .clone();

            Box::new(dark_sky::Provider::new(config))
        }

        WeatherProviderName::Dummy => {
            Box::new(dummy::Provider::new())
        }

        WeatherProviderName::OpenWeatherMap => {
            let config = config
                .providers
                .settings
                .open_weather_map.as_ref().expect("OpenWeatherMap provider has not been configured - please create the [providers.settings.open-weather-map] section.")
                .clone();

            Box::new(open_weather_map::Provider::new(config))
        }
    }
}