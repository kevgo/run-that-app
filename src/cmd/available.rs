use super::run::load_or_install;
use crate::cli::AppVersion;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

pub fn available(app_version: AppVersion, include_path: bool, output: &dyn Output) -> Result<ExitCode> {
    match load_or_install(app_version, include_path, output)? {
        Some(_) => Ok(ExitCode::SUCCESS),
        None => Ok(ExitCode::FAILURE),
    }
}
