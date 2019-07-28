use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::LevelFilter;

use lib_app_config::Config;

/// Initializes the global application logger.
pub fn init_logger(config: &Config) {
    let colors = ColoredLevelConfig::new()
        .info(Color::Blue)
        .debug(Color::BrightMagenta)
        .trace(Color::BrightMagenta);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(if config.app.debug { LevelFilter::Trace } else { LevelFilter::Error })
        .level_for("hyper", LevelFilter::Off)
        .level_for("mio", LevelFilter::Off)
        .level_for("reqwest", LevelFilter::Off)
        .level_for("tokio_reactor", LevelFilter::Off)
        .level_for("want", LevelFilter::Off)
        .level_for("winit", LevelFilter::Off)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}