#[macro_use]
extern crate log;

use forecast::{ApiClient, ApiResponse, DataPoint, ExtendBy, ForecastRequestBuilder, Icon, Lang, Units};
use reqwest::Client;
use serde::Deserialize;

use lib_service_common::{Weather, WeatherForecast, WeatherIcon, WeatherService};

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub api_key: String,
    pub latitude: f64,
    pub longitude: f64,
}

pub struct Service {
    config: Config,
    client: Client,
}

impl Service {
    pub fn new(config: Config) -> Self {
        info!("Initializing DarkSky ({:?})", config);

        Self { config, client: Client::new() }
    }
}

impl Service {
    fn weather(&self) -> ApiResponse {
        let request = ForecastRequestBuilder::new(&self.config.api_key, self.config.latitude, self.config.longitude)
            .extend(ExtendBy::Hourly)
            .lang(Lang::English)
            .units(Units::Auto)
            .build();

        ApiClient::new(&self.client)
            .get_forecast(request)
            .unwrap()
            .json()
            .unwrap()
    }
}

impl WeatherService for Service {
    fn current(&mut self) -> Weather {
        self.weather()
            .currently
            .map(utils::weather)
            .unwrap_or_else(Weather::default)
    }

    fn forecast(&mut self) -> Option<WeatherForecast> {
        unimplemented!()
    }
}

mod utils {
    use super::*;

    pub fn weather(dp: DataPoint) -> Weather {
        Weather {
            temperature: dp.temperature.map(|v| v as f32),
            pressure: dp.pressure.map(|v| v as i32),
            humidity: dp.humidity.map(|v| (v * 100.0) as i32),
            wind_speed: dp.wind_speed.map(|v| v as f32),

            icon: dp.icon.as_ref().map(|icon| {
                match icon {
                    Icon::ClearDay | Icon::ClearNight => WeatherIcon::Sun,
                    Icon::Rain => WeatherIcon::Rain,
                    Icon::Snow | Icon::Sleet => WeatherIcon::Snow,
                    Icon::Wind => WeatherIcon::SunWithCloud,
                    Icon::Fog => WeatherIcon::Fog,
                    Icon::Cloudy | Icon::PartlyCloudyDay | Icon::PartlyCloudyNight => WeatherIcon::SunWithCloud,
                    Icon::Hail | Icon::Thunderstorm | Icon::Tornado => WeatherIcon::Thunderstorm,
                }
            }),
        }
    }
}