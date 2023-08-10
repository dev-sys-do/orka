use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// CNI Spec: [Error](https://www.cni.dev/docs/spec/#error)
#[derive(Debug, Deserialize, Serialize)]
pub struct CniError {
    #[serde(rename = "cniVersion")]
    cni_version: String,
    code: CniErrorCode,
    msg: String,
    details: String,
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
