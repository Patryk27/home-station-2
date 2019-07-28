use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub debug: bool,
    pub language: Language,
}

#[derive(Deserialize, Debug)]
pub enum Language {
    #[serde(rename = "pl")]
    Polish,
}