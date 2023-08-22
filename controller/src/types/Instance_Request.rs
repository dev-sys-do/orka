use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct Instance_Request {
    #[validate(length(min = 1))]
    pub workload_id: String,
}
