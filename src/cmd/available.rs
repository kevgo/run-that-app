use super::run::load_or_install;
use crate::config::AppName;
use crate::config::Versions;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

pub fn available(app: &AppName, versions: &Versions, include_path: bool, output: &dyn Output) -> Result<ExitCode> {
    for version in versions.iter() {
        if load_or_install(app, version, include_path, output)?.is_some() {
            return Ok(ExitCode::SUCCESS);
        }
    }
    Ok(ExitCode::FAILURE)
}
