use crate::config::{Config, FILE_NAME};
use crate::Result;
use std::process::ExitCode;

pub fn setup() -> Result<ExitCode> {
    Config::create()?;
    println!("Created file {FILE_NAME}");
    Ok(ExitCode::SUCCESS)
}
