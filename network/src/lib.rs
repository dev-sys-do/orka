pub mod cni;

use cni::error::CniError;

pub fn add() -> Result<(), CniError> {
    println!("ADD method");
    Ok(())
}

pub fn delete() -> Result<(), CniError> {
    println!("DELETE method");
    Ok(())
}

pub fn check() -> Result<(), CniError> {
    println!("CHECK method");
    Ok(())
}

pub fn version() -> Result<(), CniError> {
    println!("VERSION method");
    Ok(())
}
