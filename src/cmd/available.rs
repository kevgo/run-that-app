use super::run::load_or_install;
use crate::apps;
use crate::config::AppName;
use crate::config::RequestedVersions;
use crate::config::Version;
use crate::output;
use crate::platform;
use crate::Result;
use std::process::ExitCode;

pub fn available(app_name: &AppName, version: Option<Version>, verbose: bool) -> Result<ExitCode> {
    let apps = apps::all();
    let app = apps.lookup(app_name)?;
    let log = output::new(verbose);
    let platform = platform::detect(log)?;
    let versions = RequestedVersions::determine(app_name, version, &apps)?;
    for version in versions {
        if load_or_install(app, &version, platform, log)?.is_some() {
            return Ok(ExitCode::SUCCESS);
        }
    }
    Ok(ExitCode::FAILURE)
}
