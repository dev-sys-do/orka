use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct Workload_Request {
    pub version: String,
    pub workload: Workload,
}
#[derive(Debug, Validate, Deserialize)]
pub struct Workload {
    #[validate(length(min = 1))]
    pub kind: String,

    #[validate(length(min = 1))]
    pub name: String,

    pub environment: Vec<String>,

    pub registry: String,

    #[validate(length(min = 1))]
    pub image: String,

    pub port: String,

    pub network: Vec<String>,
}
