use std::sync::mpsc::{channel, Receiver, Sender};

use lib_backend_common::WeatherProvider;

use super::WeatherQuery;

pub struct WeatherQueryExecutor {
    provider: Box<dyn WeatherProvider>,
    rx: Receiver<WeatherQuery>,
}

impl WeatherQueryExecutor {
    pub(crate) fn new(provider: Box<dyn WeatherProvider>) -> (Sender<WeatherQuery>, Self) {
        let (tx, rx) = channel();

        (tx, Self { provider, rx })
    }

    pub fn start(mut self) {
        for query in self.rx.iter() {
            trace!("Processing query: {:?}", query);

            match query {
                WeatherQuery::GetCurrent { tx } => {
                    tx.send(self.provider.current()).unwrap();
                }

                WeatherQuery::GetForecast { tx } => {
                    tx.send(self.provider.forecast()).unwrap();
                }
            }
        }
    }
}