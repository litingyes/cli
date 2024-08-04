use std::env::current_dir;
use std::fmt;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};

use config::{Config, File};
use serde::{Deserialize, Serialize};

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
    let mut global_config_path = PathBuf::from(dirs::home_dir().unwrap());
    global_config_path.push("liting");
    let mut local_config_path = PathBuf::from(current_dir().unwrap());
    local_config_path.push("liting");

    let config = Config::builder()
        .add_source(File::with_name("src/config/liting.default"))
        .add_source(
            File::from(Path::new(
                global_config_path
                    .to_str()
                    .expect("Load global config file path failed."),
            ))
            .required(false),
        )
        .add_source(
            File::from(Path::new(
                local_config_path
                    .to_str()
                    .expect("Load local config file path failed."),
            ))
            .required(false),
        )
        .build()
        .expect("Get config failed");

    config
        .try_deserialize::<LitingConfig>()
        .expect("Deserialize config failed")
}
