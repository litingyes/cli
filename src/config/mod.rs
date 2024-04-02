extern crate dirs;

use console::style;
use std::path::PathBuf;
use std::{env, fs};
use toml_edit::DocumentMut;

const CONFIG_FILE: &'static str = "liting.toml";

fn get_global_config() -> Result<DocumentMut, ()> {
    if let Some(global_config_dir) = dirs::config_dir() {
        let mut path_buf = PathBuf::new();
        path_buf.push(global_config_dir);
        path_buf.push(CONFIG_FILE);
        let global_config_file_path = path_buf.as_path();
        println!(
            "The global config file path: {}.",
            style(global_config_file_path.to_string_lossy()).yellow()
        );

        if global_config_file_path.exists() {
            match fs::read_to_string(global_config_file_path) {
                Ok(global_config_str) => {
                    let global_config = global_config_str
                        .parse::<DocumentMut>()
                        .expect("invalid global config");

                    Ok(global_config)
                }
                Err(e) => {
                    eprintln!("Error read global config file: {}", e);
                    Err(())
                }
            }
        } else {
            fs::File::create_new(global_config_file_path)
                .expect("Error create global config file.");
            println!("Cannot find existing global config file, so create one.");
            Ok(DocumentMut::new())
        }
    } else {
        eprintln!("Error read global config file.");
        Err(())
    }
}

fn get_local_config() -> Result<DocumentMut, ()> {
    if let Some(local_config_dir) = env::current_dir() {
        let mut path_buf = PathBuf::new();
        path_buf.push(local_config_dir);
        path_buf.push(CONFIG_FILE);
        let local_config_file_path = path_buf.as_path();
        println!(
            "The local config file path: {}",
            style(local_config_file_path.to_string_lossy()).yellow()
        );

        if local_config_file_path.exists() {
            match fs::read_to_string(local_config_file_path) {
                Ok(local_config_str) => {
                    let local_config = local_config_str
                        .parse::<DocumentMut>()
                        .expect("invalid local config.");

                    Ok(local_config)
                }
                Err(e) => {
                    eprintln!("Error read local config file: {}.", e);
                    Err(())
                }
            }
        } else {
            println!("Cannot find existing local config file.");
            Err(())
        }
    } else {
        eprintln!("Error read global config file.");
        Err(())
    }
}

pub fn list_config() {
    let global_config = get_global_config();
    match global_config {
        Ok(config) => {
            println!("{}", config.to_string())
        }
        Err(_e) => {
            eprintln!("Error list config")
        }
    }

    let local_config = get_local_config();
    match local_config {
        Ok(config) => {
            println!("{}", config.to_string())
        }
        Err(_e) => {
            eprintln!("Error list config")
        }
    }
}
