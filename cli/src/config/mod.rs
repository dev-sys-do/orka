use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::sync::{Arc, Mutex, MutexGuard};

use serde::{Deserialize, Serialize};
use url::{ParseError, Url};
use validator::Validate;

use crate::{APP_CONFIG, DISPLAY};

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Config {
    #[serde(rename = "orkaUrl")]
    #[validate(url)]
    pub orka_url: String,
    #[serde(rename = "orkaPort", default = "Config::get_default_port")]
    #[validate(range(min = 0, max = 65535))]
    pub orka_port: u16,
}

impl Config {
    /// Initialise the configuration
    ///
    /// Reads file from disk or create it from the default config function
    fn new() -> Self {
        let config_file_location = Config::get_config_path();
        if !config_file_location.exists() {
            Config::generate_default_config()
        }

        let file = match fs::File::open(config_file_location) {
            Ok(file) => file,
            Err(e) => {
                DISPLAY.print_error(&format!("Error reading configuration file: {}", e));
                exit(-1);
            }
        };

        match serde_yaml::from_reader(file) {
            Ok(conf) => conf,
            Err(e) => {
                DISPLAY.print_error(&format!("Error parsing configuration file: {}", e));
                exit(-1)
            }
        }
    }

    /// Save the current configuration to disk
    pub fn save(&self) {
        match self.validate() {
            Ok(_) => {}
            Err(e) => {
                DISPLAY.print_error(&format!("Error validating configuration: {}", e));
                exit(-1)
            }
        }

        let file_location = Config::get_config_path();
        match serde_yaml::to_string(self) {
            Err(_) => DISPLAY.print_error("Failed to save config !"),
            Ok(config) => match fs::write(file_location, config) {
                Ok(_) => (),
                Err(_) => DISPLAY.print_error("Failed to save config !"),
            },
        }
    }

    /// Generate the default configuration
    ///
    /// Create the directory structure and writes the default configuration in the orka config file
    fn generate_default_config() {
        let file_location = Config::get_config_path();
        match fs::create_dir_all(file_location.parent().unwrap()) {
            Ok(_) => (),
            Err(e) => {
                DISPLAY.print_error(&format!(
                    "Error creating configuration file structure: {}",
                    e
                ));
                exit(-1);
            }
        }

        match fs::write(file_location, "orkaUrl: http://localhost\norkaPort: 3000\n") {
            Ok(_) => (),
            Err(e) => {
                DISPLAY.print_error(&format!("Error creating configuration file: {}", e));
                exit(-1);
            }
        }
    }

    /// Gather the config path
    ///
    /// Generates it from the user' home
    fn get_config_path() -> PathBuf {
        // FIXME let's hope the home env is defined
        let home = home::home_dir().unwrap();
        home.join(".config").join("orka").join("config.yaml")
    }

    /// Adds our port to the user given url
    ///
    /// Transform the given URL with the given port
    fn add_port_to_url(new_url: &str, new_port: u16) -> Result<String, ParseError> {
        let mut url = Url::parse(new_url)?;
        url.set_port(Some(new_port))
            .map_err(|_| ParseError::EmptyHost)?;
        Ok(url.into())
    }

    pub fn get_url_and_port(new_url: &str, new_port: u16) -> String {
        match Self::add_port_to_url(new_url, new_port) {
            Ok(url) => url,
            Err(e) => {
                DISPLAY.print_error(&format!("Error parsing URL and port: {}", e));
                exit(-1)
            }
        }
    }

    /// Wrap the config struct into an Arc<Mutex>>
    pub fn new_wrapped() -> Arc<Mutex<Config>> {
        let config = Config::new();
        Arc::new(Mutex::new(config))
    }

    /// Change the orka_url parameter to the given input
    pub fn set_orka_url(&mut self, new_url: &str) {
        self.orka_url = new_url.to_string();
    }

    pub fn set_orka_port(&mut self, new_port: u16) {
        self.orka_port = new_port;
    }

    /// Get a locked MutexGuard from the config struct
    pub fn get_config_lock<'a>() -> MutexGuard<'a, Self> {
        match APP_CONFIG.lock() {
            Ok(config) => config,
            Err(_) => {
                DISPLAY.print_error("Cannot lock the config !");
                exit(-1);
            }
        }
    }

    fn get_default_port() -> u16 {
        3000
    }
}
