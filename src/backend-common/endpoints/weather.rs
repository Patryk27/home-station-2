use std::fmt::{Display, Error, Formatter};
use std::fmt;
use std::str::FromStr;

pub trait WeatherProvider: Send {
    fn name(&self) -> &'static str;
    fn current(&mut self) -> WeatherStatus;
    fn forecast(&mut self) -> Option<WeatherForecast>;
}

#[derive(Default, Debug)]
pub struct WeatherStatus {
    pub temperature: Option<f32>,
    pub pressure: Option<i32>,
    pub humidity: Option<i32>,
    pub wind_speed: Option<f32>,
    pub icon: Option<WeatherIcon>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum WeatherIcon {
    Cloud,
    Fog,
    HeavyCloud,
    HeavyRain,
    Rain,
    Snow,
    Sun,
    SunWithCloud,
    Thunderstorm,
}

#[derive(Default, Debug)]
pub struct WeatherForecast {
    pub today_morning: Option<WeatherStatus>,
    pub today_midday: Option<WeatherStatus>,
    pub today_evening: Option<WeatherStatus>,
    pub tomorrow: Option<WeatherStatus>,
    pub in_two_days: Option<WeatherStatus>,
    pub in_three_days: Option<WeatherStatus>,
}

impl Display for WeatherIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            WeatherIcon::Cloud => "cloud",
            WeatherIcon::Fog => "fog",
            WeatherIcon::HeavyCloud => "heavy-cloud",
            WeatherIcon::HeavyRain => "heavy-rain",
            WeatherIcon::Rain => "rain",
            WeatherIcon::Snow => "snow",
            WeatherIcon::Sun => "sun",
            WeatherIcon::SunWithCloud => "sun-with-cloud",
            WeatherIcon::Thunderstorm => "thunderstorm",
        })
    }
}