use super::error::CniErrorCode;
use std::str::FromStr;

#[derive(Debug)]
pub enum CniMethod {
    Add,
    Delete,
    Check,
    Version,
}

impl FromStr for CniMethod {
    type Err = CniErrorCode;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ADD" => Ok(CniMethod::Add),
            "DEL" => Ok(CniMethod::Delete),
            "CHECK" => Ok(CniMethod::Check),
            "VERSION" => Ok(CniMethod::Version),
            _ => Err(CniErrorCode::InvalidEnvironmentVariables),
        }
    }
}
