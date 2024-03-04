use crate::config::{AppName, Version};
use crate::Output;
use crate::Result;
use std::process::ExitCode;

use super::run::load_or_install;

pub fn which(app: &AppName, version: &Version, include_path: bool, output: &dyn Output) -> Result<ExitCode> {
    if let Some(executable) = load_or_install(app, version, include_path, output)? {
        println!("{}", executable.0.to_string_lossy());
    }
    Ok(ExitCode::SUCCESS)
}
