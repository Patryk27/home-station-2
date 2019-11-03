use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub airly: Option<lib_service_providers::airly::Config>,
    pub dark_sky: Option<lib_service_providers::dark_sky::Config>,
    pub open_weather_map: Option<lib_service_providers::open_weather_map::Config>,
}