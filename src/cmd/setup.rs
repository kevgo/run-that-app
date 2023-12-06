use crate::config::FILE_NAME;
use crate::{config, Result};
use std::process::ExitCode;

pub fn setup() -> Result<ExitCode> {
    config::create()?;
    println!("Created file {FILE_NAME}");
    Ok(ExitCode::SUCCESS)
}
