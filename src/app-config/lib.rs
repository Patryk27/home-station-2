use std::fs;
use std::path::Path;

use serde::Deserialize;

pub mod app;
pub mod display;
pub mod providers;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub app: app::Config,
    pub display: display::Config,
    pub providers: providers::Config,
}

impl Config {
    /// Loads HomeStation's configuration from specified file.
    pub fn from_file(path: &Path) -> Self {
        let config = fs::read_to_string(path).expect(
            "failed to open the configuration file: "
        );

        toml::from_str(&config).expect(
            "failed to parse the configuration file: "
        )
    }
}