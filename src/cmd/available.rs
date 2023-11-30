use crate::cli::RequestedApp;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

use super::run::load_or_install;

pub fn available(requested_app: RequestedApp, include_global: bool, output: &dyn Output) -> Result<ExitCode> {
    if load_or_install(requested_app, include_global, output)?.is_some() {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}
