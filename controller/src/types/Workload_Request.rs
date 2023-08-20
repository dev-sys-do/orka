pub mod Workload_Request;

#[derive(Debug, Validate, Deserialize)]
pub struct Workload_Request{
    pub version: string,
    pub workload: Workload,

    
}

pub struct Workload{
    #[validate(length(min = 1))]
    kind: String,

    #[validate(length(min = 1))]
    name: String,
    
    environment: Vec<String>,
    
    registry: String,
    
    #[validate(length(min = 1))]
    image: String,
    
    port: String,
    
    network: Vec<String>
}   