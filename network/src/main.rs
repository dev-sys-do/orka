use orka_cni::cni::{
    error::{CniError, CniErrorCode},
    method::CniMethod,
};
use std::{process, str::FromStr};

fn main() {
    // we don't return Err(CniError) in main()
    // because it uses the debug trait instead of Display
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1)
    }
}

fn run() -> Result<(), CniError> {
    let method = std::env::var("CNI_METHOD")
        .map(|str| CniMethod::from_str(&str))
        .map_err(|_| {
            CniError::new(
                Some(CniErrorCode::InvalidEnvironmentVariables),
                "Missing CNI_METHOD environment variable",
                None,
            )
        })?
        .map_err(|_| {
            CniError::new(
                Some(CniErrorCode::InvalidEnvironmentVariables),
                "Invalid CNI_METHOD environment variable",
                None,
            )
        })?;

    match method {
        CniMethod::Add => orka_cni::add(),
        CniMethod::Delete => orka_cni::delete(),
        CniMethod::Check => orka_cni::check(),
        CniMethod::Version => orka_cni::version(),
    }?;

    Ok(())
}
