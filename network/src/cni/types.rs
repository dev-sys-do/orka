/// see [cni/docs/spec/error](https://www.cni.dev/docs/spec/#error)
/// see [cni/pkg/types/types.go](https://github.com/containernetworking/cni/blob/main/pkg/types/types.go#L153)
use std::fmt;

#[derive(Debug)]
pub struct Error {
    code: Code,
    msg: String,
    details: String,
}

#[derive(Debug)]
pub enum Code {
    ErrUnknown = 0,
    ErrIncompatibleCNIVersion = 1,
    ErrUnsupportedField = 2,
    ErrUnknownContainer = 3,
    ErrInvalidEnvironmentVariables = 4,
    ErrIOFailure = 5,
    ErrDecodingFailure = 6,
    ErrInvalidNetworkConfig = 7,
    ErrInvalidNetNS = 8,
    ErrTryAgainLater = 11,
    ErrInternal = 999,
}

impl Error {
    pub fn new(code: Code, msg: String, details: String) -> Self {
        Error { code, msg, details }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
        write!(f, "{:?} + {} + {}", self.code, self.msg, self.details)
    }
}
