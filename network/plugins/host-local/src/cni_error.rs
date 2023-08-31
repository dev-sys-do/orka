use json::{object, JsonValue};

pub enum CNIErrorCode {
    IncompatibleCNIVersion = 1,
    UnsupportedFieldInNetworkConfig = 2,
    ContainerUnknownOrDoesntExist = 3,
    InvalidNecessaryEnvVars = 4,
    IOFailure = 5,
    FailedToDecodeContent = 6,
    InvalidNetworkConfig = 7,
    TryAgainLater = 11
}

pub fn output_error(msg: &String, details: &String, code: CNIErrorCode, cni_version: &String) {
    let json_error: JsonValue = object!{
        cniVersion: cni_version.clone(),
        code: code as u8,
        msg: msg.clone(),
        details: details.clone()
    };

    println!("{}", json_error);
}