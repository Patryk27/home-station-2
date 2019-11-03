use std::fmt;

pub trait WeatherService: Send {
    fn current(&mut self) -> Weather;
    fn forecast(&mut self) -> Option<WeatherForecast>;
}

#[derive(Default, Debug)]
pub struct Weather {
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
    pub today_morning: Option<Weather>,
    pub today_midday: Option<Weather>,
    pub today_evening: Option<Weather>,
    pub tomorrow: Option<Weather>,
    pub in_two_days: Option<Weather>,
    pub in_three_days: Option<Weather>,
}

impl fmt::Display for WeatherIcon {
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