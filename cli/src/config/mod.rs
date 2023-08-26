use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::DISPLAY;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "orkaUrl")]
    pub orka_url: String,
}

impl Config {
    /// Initialise the configuration
    fn new() -> Self {
        let config_file_location = Config::get_config_path();
        if !config_file_location.exists() {
            Config::generate_default_config()
        }

        let file = match fs::File::open(config_file_location) {
            Ok(file) => file,
            Err(e) => {
                println!("{}", e);
                exit(-1);
            }
        };

        let config: Config = match serde_yaml::from_reader(file) {
            Ok(conf) => {
                let mut final_conf: Config = conf;
                final_conf.orka_url = final_conf.orka_url + ":3000/";
                return final_conf;
            },
            Err(e) => {
                println!("Error parsing configuration file: {}", e);
                exit(-1)
            }
        };
    }

    /// Save the current configuration
    pub fn save(&self) {
        let file_location = Config::get_config_path();
        match serde_yaml::to_string(self) {
            Err(_) => DISPLAY.print_error("Failed to save config !"),
            Ok(config) => {
                match fs::write(file_location, config) {
                    Ok(_) => (),
                    Err(_) => DISPLAY.print_error("Failed to save config !"),
                }
            }
        }
    }

    /// Generate the default configuration
    fn generate_default_config() {
        let file_location = Config::get_config_path();
        match fs::create_dir_all(file_location.parent().unwrap()) {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                exit(-1);
            }
        }

        match fs::write(file_location, "orkaUrl: http://localhost\n") {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                exit(-1);
            }
        }
    }

    /// Gather the config path
    fn get_config_path() -> PathBuf {
        // FIXME let's hope the home env is defined
        let home = home::home_dir().unwrap();
        home.join(".config").join("orka").join("config.yaml")
    }

    pub fn new_wrapped() -> Arc<Mutex<Config>> {
        let config = Config::new();
        Arc::new(Mutex::new(config))
    }

    pub fn set_orka_url(&mut self, new_url: &str) {
        self.orka_url = new_url.to_string();
    }
}
