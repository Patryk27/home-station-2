use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub debug: bool,
    pub language: Language,

    pub air_service: AirServiceName,
    pub time_service: TimeServiceName,
    pub weather_service: WeatherServiceName,
}

#[derive(Deserialize, Debug)]
pub enum Language {
    #[serde(rename = "pl")]
    Polish,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum AirServiceName {
    Dummy,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TimeServiceName {
    Chrono,
    Dummy,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum WeatherServiceName {
    DarkSky,
    Dummy,
    OpenWeatherMap,
}