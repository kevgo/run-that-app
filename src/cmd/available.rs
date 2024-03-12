use super::run::load_or_install;
use crate::apps;
use crate::config::AppName;
use crate::config::RequestedVersions;
use crate::config::Version;
use crate::output::Output;
use crate::platform;
use crate::Result;
use std::process::ExitCode;

pub fn available(app_name: &AppName, version: Option<Version>, log: Option<String>) -> Result<ExitCode> {
    let apps = apps::all();
    let app = apps.lookup(app_name)?;
    let output = Output { verbose: log };
    let platform = platform::detect(&output)?;
    let versions = RequestedVersions::determine(app_name, version, &apps)?;
    for version in versions {
        if load_or_install(app, &version, platform, &output)?.is_some() {
            return Ok(ExitCode::SUCCESS);
        }
    }
    Ok(ExitCode::FAILURE)
}
