use lib_service::{TimeClient, WeatherClient};

pub struct State {
    pub time: TimeClient,
    pub weather: WeatherClient,
}