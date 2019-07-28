use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub months: [String; 12],
    pub weekdays: [String; 7],
}