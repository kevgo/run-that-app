use super::run::load_or_install;
use crate::config::AppName;
use crate::config::Version;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

pub fn available(app: &AppName, version: &Version, include_path: bool, output: &dyn Output) -> Result<ExitCode> {
    match load_or_install(app, version, include_path, output)? {
        Some(_) => Ok(ExitCode::SUCCESS),
        None => Ok(ExitCode::FAILURE),
    }
}
