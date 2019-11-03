#[macro_use]
extern crate log;

use lib_service_common::{Time, TimeService, Weather, WeatherForecast, WeatherIcon, WeatherService};

pub struct Provider;

impl Provider {
    pub fn new() -> Self {
        info!("Initializing dummy");

        Self
    }
}

impl TimeService for Provider {
    fn current(&mut self) -> Time {
        Time {
            hour: 12,
            minute: 0,
            second: 0,

            day: 1,
            month: 1,
            year: 1980,
        }
    }
}

impl WeatherService for Provider {
    fn current(&mut self) -> Weather {
        Weather {
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