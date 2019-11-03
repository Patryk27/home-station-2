use std::sync::mpsc::Sender;

use lib_service_common::{Weather, WeatherForecast};

#[derive(Debug)]
pub enum WeatherRequest {
    GetWeather {
        tx: Sender<Weather>,
    },

    GetWeatherForecast {
        tx: Sender<Option<WeatherForecast>>,
    },
}