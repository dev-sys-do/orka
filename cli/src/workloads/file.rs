use serde::{Serialize, Deserialize};
use std::fs;
use crate::workloads::container::{WorkloadContainerFile};
use crate::workloads::network::{WorkloadNetworkFile};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Container,
    Network,
}


#[derive(Serialize, Deserialize)]
struct TestFile {
    workload: TestKind
}


#[derive(Serialize, Deserialize)]
struct TestKind {
    kind: Kind
}



pub fn read_file(filepath : &str) -> Option<serde_json::Value> {
    // read file
    let contents = fs::read_to_string(filepath)
    .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    // convert file to yaml => take only the kind to know what type of Container we are reading
    let yaml: TestFile = serde_yaml::from_str(&contents).unwrap();


    // check type of workload
    if yaml.workload.kind == Kind::Network {

        // read file into corresponding struct
        let network : WorkloadNetworkFile = serde_yaml::from_str(&contents).unwrap();

        // read verified yaml structure to json
        let networkstring : String = serde_yaml::to_string(&network).unwrap();
        println!("String:\n{:?}", networkstring);
        let json : serde_json::Value = serde_yaml::from_str(&networkstring).unwrap();
        println!("Json:\n{:?}", json);

        // return json
        return Some(json);

    }
    else if yaml.workload.kind == Kind::Container {

        // read file into corresponding struct
        let container : WorkloadContainerFile = serde_yaml::from_str(&contents).unwrap();

        // read verified yaml structure to json
        let containerstring : String = serde_yaml::to_string(&container).unwrap();
        println!("String:\n{:?}", containerstring);
        let json : serde_json::Value = serde_yaml::from_str(&containerstring).unwrap();
        println!("Json:\n{:?}", json);

        // return json
        return Some(json);
    }

    return None;
}