use crate::cli::AppVersion;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

use super::run::load_or_install;

pub fn which(app_version: AppVersion, include_path: bool, output: &dyn Output) -> Result<ExitCode> {
    if let Some(executable) = load_or_install(app_version, include_path, output)? {
        println!("{}", executable.0.to_string_lossy());
    }
    Ok(ExitCode::SUCCESS)
}
