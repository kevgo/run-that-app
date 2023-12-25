use super::run::load_or_install;
use crate::cli::RequestedApp;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

pub fn available(requested_app: RequestedApp, include_path: bool, output: &dyn Output) -> Result<ExitCode> {
    match load_or_install(requested_app, include_path, output)? {
        Some(_) => Ok(ExitCode::SUCCESS),
        None => Ok(ExitCode::FAILURE),
    }
}
