use std::fs;
use std::process::exit;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "orkaUrl")]
    orka_url: String
}


pub fn get_config() -> Config {
    let home = home::home_dir().unwrap();
    let config_location = format!("{}/.config/orka", home.as_path().display());
    let config_file_location = format!("{}/config.yaml", config_location);
    let file = match fs::File::open(config_file_location.clone()) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                generate_default_config(config_location, config_file_location)
            }
            else {
                println!("{}", e);
                exit(-1);
            }
        }
    };


    let config: Config = match serde_yaml::from_reader(file) {
        Ok(conf)=> conf,
        Err(e)=> {
            println!("Error parsing configuration file: {}", e);
            exit(-1)
        }
    };

    config
}

fn generate_default_config(config_location: String, config_file_location: String) -> fs::File {
    match fs::create_dir_all(config_location){
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            exit(-1);
        }
    }
    match fs::write(config_file_location.clone(), "orka_url: http://localhost\n") {
        Ok(..) => match fs::File::open(config_file_location) {
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                exit(-1);
            }
        },
        Err(e) => {
            println!("{}", e);
            exit(-1);
        }
    }
}
