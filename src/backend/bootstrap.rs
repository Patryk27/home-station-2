use std::thread::spawn;

use lib_backend_common::{TimeProvider, WeatherProvider};

use crate::{Backend, TimeQueryExecutor, WeatherQueryExecutor};

impl Backend {
    pub fn new(
        time_provider: Box<dyn TimeProvider>,
        weather_provider: Box<dyn WeatherProvider>,
    ) -> Self {
        // Spawn time query executor
        let (time_query_tx, time_query_executor) = TimeQueryExecutor::new(time_provider);

        spawn(move || {
            time_query_executor.start()
        });

        // Spawn weather query executor
        let (weather_query_tx, weather_query_executor) = WeatherQueryExecutor::new(weather_provider);

        spawn(move || {
            weather_query_executor.start();
        });

        Self { time_query_tx, weather_query_tx }
    }
}