use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub api_key: String,
    pub latitude: f64,
    pub longitude: f64,
}