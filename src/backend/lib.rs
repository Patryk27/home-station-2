#[macro_use]
extern crate log;

use std::sync::mpsc::{channel, Sender};

use lib_backend_common::{TimeStatus, WeatherForecast, WeatherStatus};

pub use self::{
    endpoints::*,
    poll::*,
};

mod bootstrap;

mod endpoints;
mod poll;

#[derive(Clone)]
pub struct Backend {
    time_query_tx: Sender<TimeQuery>,
    weather_query_tx: Sender<WeatherQuery>,
}

impl Backend {
    pub fn time(&self) -> TimeStatus {
        let (tx, rx) = channel();

        self.time_query_tx
            .send(TimeQuery::GetCurrent { tx })
            .unwrap();

        rx.recv().unwrap()
    }

    pub fn weather(&self) -> WeatherStatus {
        let (tx, rx) = channel();

        self.weather_query_tx
            .send(WeatherQuery::GetCurrent { tx })
            .unwrap();

        rx.recv().unwrap()
    }

    pub fn weather_forecast(&self) -> Option<WeatherForecast> {
        let (tx, rx) = channel();

        self.weather_query_tx
            .send(WeatherQuery::GetForecast { tx })
            .unwrap();

        rx.recv().unwrap()
    }

    pub fn poll<F>(&self) -> PollBuilder<F> {
        PollBuilder::new(self.clone())
    }
}