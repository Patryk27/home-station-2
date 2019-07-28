use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub air: AirProviderName,
    pub time: TimeProviderName,
    pub weather: WeatherProviderName,
    pub settings: Settings,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum AirProviderName {
    Dummy,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TimeProviderName {
    Chrono,
    Dummy,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum WeatherProviderName {
    DarkSky,
    Dummy,
    OpenWeatherMap,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    pub airly: Option<lib_backend_providers::airly::Config>,
    pub chrono: Option<lib_backend_providers::chrono::Config>,
    pub dark_sky: Option<lib_backend_providers::dark_sky::Config>,
    pub open_weather_map: Option<lib_backend_providers::open_weather_map::Config>,
}