use crate::cli::RequestedApp;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

use super::run::load_or_install;

pub fn show_path(requested_app: RequestedApp, include_global: bool, output: &dyn Output) -> Result<ExitCode> {
    if let Some(executable) = load_or_install(requested_app, include_global, output)? {
        println!("{}", executable.0.to_string_lossy());
    }
    Ok(ExitCode::SUCCESS)
}
