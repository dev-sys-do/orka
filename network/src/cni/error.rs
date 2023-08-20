// // CNI Spec: [Error](https://www.cni.dev/docs/spec/#error)

// use std::fmt;

// pub struct Error {
//     cni_version: String,
//     code: Code,
//     msg: String,
//     details: String,
// }

// /// see [cni/pkg/types/types.go](https://github.com/containernetworking/cni/blob/main/pkg/types/types.go#L153)
// #[derive(Debug)]
// enum Code {
//     Unknown = 0,
//     IncompatibleCNIVersion = 1,
//     UnsupportedField = 2,
//     UnknownContainer = 3,
//     InvalidEnvironmentVariables = 4,
//     IOFailure = 5,
//     DecodingFailure = 6,
//     InvalidNetworkConfig = 7,
//     InvalidNetNS = 8,
//     TryAgainLater = 11,
//     Internal = 999,
// }

// impl Error {
//     pub fn new(cni_version: String, code: Code, msg: String, details: Option<String>) -> Self {
//         Error {
//             cni_version,
//             code,
//             msg,
//             details: details.unwrap_or_default(),
//         }
//     }
// }

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
//         write!(
//             f,
//             "{} + {:?} + {} + {}",
//             self.cni_version, self.code, self.msg, self.details
//         )
//     }
// }
