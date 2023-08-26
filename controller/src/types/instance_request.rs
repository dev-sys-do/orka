use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct InstanceRequest {
    #[validate(length(min = 1))]
    pub workload_id: String,
}
