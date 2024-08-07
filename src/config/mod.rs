use std::env::current_dir;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use config::{Config, File};
use console::style;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LitingConfigCommitType {
    pub key: String,
    pub description: String,
}
impl Display for LitingConfigCommitType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            style(self.key.to_string()).bright().bold(),
            style(self.description.to_string()).dim().italic()
        )
    }
}
impl fmt::Debug for LitingConfigCommitType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Key: {} -- Description: {}", self.key, self.description)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LitingConfigCommit {
    pub types: Vec<LitingConfigCommitType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LitingConfig {
    pub commit: LitingConfigCommit,
}

pub fn get_config() -> LitingConfig {
    let mut default_config_path = Path::new(file!()).parent().unwrap().to_path_buf();
    default_config_path.push("liting.default.json");
    let mut global_config_path = PathBuf::from(dirs::home_dir().unwrap());
    global_config_path.push("liting");
    let mut local_config_path = PathBuf::from(current_dir().unwrap());
    local_config_path.push("liting");

    let config = Config::builder()
        .add_source(File::from(default_config_path.as_path()))
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
