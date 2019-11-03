use std::path::PathBuf;

use lib_service::{TimeClient, TimeServer, WeatherClient, WeatherServer};

pub use self::bootstrap::*;

mod bootstrap;
pub mod config;

pub fn main() {
    // Load configuration
    let config = config::Config::from_file(
        &PathBuf::from("config.toml")
    );

    // Initialize logger
    init_logger(&config);

    // Initialize services
    let time = TimeClient::new(TimeServer::spawn(
        create_time_service(&config),
    ));

    let weather = WeatherClient::new(WeatherServer::spawn(
        create_weather_service(&config),
    ));

    // Start UI
    let ui_config = lib_ui_framework::Config {
        width: config.display.width,
        height: config.display.height,
    };

    lib_ui::start(
        lib_ui_framework::System::new(ui_config),
        lib_ui::State { time, weather },
    );
}