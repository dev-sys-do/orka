use serde::{Serialize, Deserialize};
use std::fs;
use crate::workloads::container::{WorkloadContainerFile};
use crate::workloads::network::{WorkloadNetworkFile, verify_network};
use thiserror::Error;
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Container,
    Network,
}


#[derive(Serialize, Deserialize, Debug)]
struct TestFile {
    workload: TestKind
}


#[derive(Serialize, Deserialize, Debug)]
struct TestKind {
    kind: Kind
}

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Workload kind must be 'container' or 'network'.")]
    UnknownWorkloadKind,
    #[error("File `{0}` not found")]
    FileNotFound(PathBuf),
    #[error("Data could not be read from file")]
    FileCouldNotBeenRead,
    #[error("`{0}`")]
    MissingFields(String),
    #[error("The ip adress `{0}` is invalid.")]
    InvalidIpAddress(String),
    #[error("The port `{0}` is outside of the allowed port range.")]
    OutsidePortRange(u32),
}


// return result
pub fn read_file(filepath : PathBuf) -> Result<serde_json::Value, CustomError> {
    // read file
    let contents = match fs::read_to_string(&filepath) {
        Ok(file) => file,
        Err(error) =>  {
            match error.kind() {
                NotFound => return Err(CustomError::FileNotFound(filepath)),
                _ => return Err(CustomError::FileCouldNotBeenRead)
            }
        }
    };

    // convert file to yaml => take only the kind to know what type of Container we are reading
    let yaml: TestFile = match serde_yaml::from_str::<TestFile>(&contents) {
        Ok(result) => result,
        Err(_) => return Err(CustomError::UnknownWorkloadKind),
    };



    // check type of workload
    match yaml.workload.kind {
        Kind::Network => {
            // read file into corresponding struct
            let network : WorkloadNetworkFile = match serde_yaml::from_str(&contents) {
                Ok(result) => result,
                Err(error) => return Err(CustomError::MissingFields(error.to_string())),
            };

            // verify fields
            match verify_network(&network.workload.egress) {
                None => (),
                Some(error) => return Err(error),
            };
            match verify_network(&network.workload.ingress){
                None => (),
                Some(error) => return Err(error),
            };

            // read verified yaml structure to json
            let networkstring : String = serde_yaml::to_string(&network).unwrap();
            // return json
            let json : serde_json::Value = serde_yaml::from_str(&networkstring).unwrap();            
            return Ok(json);
        },
        Kind::Container => {
            // read file into corresponding struct
            let container : WorkloadContainerFile = match serde_yaml::from_str(&contents) {
                Ok(result) => result,
                Err(error) => return Err(CustomError::MissingFields(error.to_string())),
            };

            // read verified yaml structure to json
            let containerstring : String = serde_yaml::to_string(&container).unwrap();
            let json : serde_json::Value = serde_yaml::from_str(&containerstring).unwrap();

            // return json
            return Ok(json);
        },
    }
}