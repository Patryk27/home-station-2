use std::sync::mpsc::Sender;

use lib_backend_common::{WeatherForecast, WeatherStatus};

#[derive(Debug)]
pub enum WeatherQuery {
    GetCurrent { tx: Sender<WeatherStatus> },
    GetForecast { tx: Sender<Option<WeatherForecast>> },
}