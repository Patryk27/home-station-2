use std::path::PathBuf;

use lib_app_config::Config;
use lib_backend::Backend;

pub use self::bootstrap::*;

mod bootstrap;

pub fn main() {
    // Load configuration
    let config = Config::from_file(
        &PathBuf::from("config.toml")
    );

    // Initialize logger
    init_logger(&config);

    // Initialize providers
    let time_provider = init_time_provider(&config);
    let weather_provider = init_weather_provider(&config);

    // Start backend
    let backend = Backend::new(
        time_provider,
        weather_provider,
    );

    // Prepare frontend
    let frontend_config = lib_frontend_core::Config {
        width: config.display.width,
        height: config.display.height,
    };

    let frontend_core = lib_frontend_core::System::new(frontend_config, backend);

    // Start frontend
    lib_frontend::start(frontend_core);
}