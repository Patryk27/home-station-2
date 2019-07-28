use owm::WeatherHub;

use lib_backend_common::{WeatherForecast, WeatherProvider, WeatherStatus};

use crate::Provider;

impl WeatherProvider for Provider {
    fn name(&self) -> &'static str {
        "open-weather-map"
    }

    fn current(&mut self) -> WeatherStatus {
        let weather = WeatherHub::new(
            hyper::Client::new(), &self.config.api_key,
        );

        let (_, weather) = weather
            .current()
            .by_name(&self.config.city, Some(&self.config.country))
            .unwrap();

        WeatherStatus {
            temperature: utils::temperature(&weather),
            pressure: utils::pressure(&weather),
            humidity: utils::humidity(&weather),
            wind_speed: utils::wind_speed(&weather),
            icon: utils::icon(&weather),
        }
    }

    fn forecast(&mut self) -> Option<WeatherForecast> {
        None
    }
}

mod utils {
    use owm::data::WeatherInfo;

    use lib_backend_common::WeatherIcon;

    pub fn temperature(weather: &WeatherInfo) -> Option<f32> {
        Some(weather.main.as_ref()?.temp? - 273.15)
    }

    pub fn pressure(weather: &WeatherInfo) -> Option<i32> {
        weather.main.as_ref()?.pressure
    }

    pub fn humidity(weather: &WeatherInfo) -> Option<i32> {
        weather.main.as_ref()?.humidity
    }

    pub fn wind_speed(weather: &WeatherInfo) -> Option<f32> {
        weather.wind.as_ref()?.speed
    }

    pub fn icon(weather: &WeatherInfo) -> Option<WeatherIcon> {
        match weather.weather.as_ref()?.get(0)?.icon.as_ref()?.as_str() {
            "01d" | "01n" => Some(WeatherIcon::Sun),
            "02d" | "02n" => Some(WeatherIcon::SunWithCloud),
            "03d" | "03n" => Some(WeatherIcon::Cloud),
            "04d" | "04n" => Some(WeatherIcon::HeavyCloud),
            "09d" | "09n" => Some(WeatherIcon::Rain),
            "10d" | "10n" => Some(WeatherIcon::HeavyRain),
            "11d" | "11n" => Some(WeatherIcon::Thunderstorm),
            "13d" | "13n" => Some(WeatherIcon::Snow),
            "50d" | "50n" => Some(WeatherIcon::Fog),

            _ => None,
        }
    }
}