use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

/// CNI Spec: [Error](https://www.cni.dev/docs/spec/#error)
#[derive(Deserialize, Serialize)]
pub struct CniError {
    #[serde(rename = "cniVersion")]
    pub cni_version: String,
    pub code: CniErrorCode,
    pub msg: String,
    pub details: String,
}

/// see [cni/pkg/types/types.go](https://github.com/containernetworking/cni/blob/main/pkg/types/types.go#L153)
#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u16)]
pub enum CniErrorCode {
    Unknown = 0,
    IncompatibleCNIVersion = 1,
    UnsupportedField = 2,
    UnknownContainer = 3,
    InvalidEnvironmentVariables = 4,
    IOFailure = 5,
    DecodingFailure = 6,
    InvalidNetworkConfig = 7,
    InvalidNetNS = 8,
    TryAgainLater = 11,
    Internal = 999,
}

impl CniError {
    pub fn new(code: Option<CniErrorCode>, msg: &str, details: Option<&str>) -> Self {
        CniError {
            cni_version: "1.0.0".to_string(),
            code: code.unwrap_or(CniErrorCode::Unknown),
            msg: msg.to_string(),
            details: details.unwrap_or_default().to_string(),
        }
    }
}

impl fmt::Display for CniError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}
