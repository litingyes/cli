use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize)]
pub struct LitingConfigType {
    pub key: String,
    pub description: String,
}
impl fmt::Debug for LitingConfigType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Key: {} -- Description: {}", self.key, self.description)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LitingConfig {
    pub types: Vec<LitingConfigType>,
}

pub fn get_config() -> LitingConfig {
    let config = Config::builder()
        .add_source(File::with_name("src/config/liting.default.toml"))
        .build()
        .expect("Get config failed");

    config
        .try_deserialize::<LitingConfig>()
        .expect("Deserialize config failed")
}
