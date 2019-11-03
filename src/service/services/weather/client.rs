use std::sync::mpsc::{channel, Sender};

use lib_service_common::{Weather, WeatherForecast};

use crate::WeatherRequest;

#[derive(Clone)]
pub struct WeatherClient {
    tx: Sender<WeatherRequest>,
}

impl WeatherClient {
    pub fn new(tx: Sender<WeatherRequest>) -> Self {
        Self { tx }
    }

    pub fn get_weather(&self) -> Weather {
        let (tx, rx) = channel();

        self.tx
            .send(WeatherRequest::GetWeather { tx })
            .unwrap();

        rx.recv().unwrap()
    }

    pub fn get_weather_forecast(&self) -> Option<WeatherForecast> {
        let (tx, rx) = channel();

        self.tx
            .send(WeatherRequest::GetWeatherForecast { tx })
            .unwrap();

        rx.recv().unwrap()
    }
}