use crate::workloads::file::remove_duplicates_array;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
enum Registry {
    #[serde(rename(deserialize = "ghcr", serialize = "Ghcr"))]
    Ghcr,
    #[serde(rename(deserialize = "docker", serialize = "Docker"))]
    Docker,
    #[serde(rename(deserialize = "podman", serialize = "Podman"))]
    Podman,
}

// default registry
impl Registry {
    fn default() -> Self {
        Registry::Docker
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Container {
    #[validate(length(min = 1))]
    #[serde(deserialize_with = "u32_to_string")]
    port: String,
    #[validate(length(min = 1))]
    name: String,
    #[serde(default, deserialize_with = "remove_duplicates_array")]
    environment: Vec<String>,
    #[serde(default, deserialize_with = "remove_duplicates_array")]
    network: Vec<String>,
    #[serde(default = "Registry::default")]
    registry: Registry,
    #[validate(length(min = 1))]
    image: String,
}

// transform port from u32 to string
pub fn u32_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Deserialize u32, then convert it to String
    let value: u32 = Deserialize::deserialize(deserializer)?;
    Ok(value.to_string())
}
