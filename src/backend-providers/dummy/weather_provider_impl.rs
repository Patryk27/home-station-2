use lib_backend_common::{WeatherForecast, WeatherIcon, WeatherProvider, WeatherStatus};

use crate::Provider;

impl WeatherProvider for Provider {
    fn name(&self) -> &'static str {
        "dummy"
    }

    fn current(&mut self) -> WeatherStatus {
        WeatherStatus {
            temperature: Some(20.0),
            pressure: Some(950),
            humidity: Some(55),
            wind_speed: Some(4.0),
            icon: Some(WeatherIcon::SunWithCloud),
        }
    }

    fn forecast(&mut self) -> Option<WeatherForecast> {
        None
    }
}