use super::run::load_or_install;
use crate::apps;
use crate::config::AppName;
use crate::config::RequestedVersions;
use crate::platform;
use crate::Output;
use crate::Result;
use std::process::ExitCode;

pub fn available(app: &AppName, versions: &RequestedVersions, output: &dyn Output) -> Result<ExitCode> {
    let apps = apps::all();
    let app = apps.lookup(app)?;
    let platform = platform::detect(output)?;
    for version in versions.iter() {
        if load_or_install(app, version, platform, output)?.is_some() {
            return Ok(ExitCode::SUCCESS);
        }
    }
    Ok(ExitCode::FAILURE)
}
