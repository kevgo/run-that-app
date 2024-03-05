use crate::config::{AppName, RequestedVersions};
use crate::Output;
use crate::Result;
use std::process::ExitCode;

use super::run::load_or_install;

pub fn which(app: &AppName, versions: &RequestedVersions, include_path: bool, output: &dyn Output) -> Result<ExitCode> {
    for version in versions.iter() {
        if let Some(executable) = load_or_install(app, version, include_path, output)? {
            println!("{}", executable.0.to_string_lossy());
            return Ok(ExitCode::SUCCESS);
        }
    }
    Ok(ExitCode::FAILURE)
}
