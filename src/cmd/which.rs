use crate::config::{AppName, RequestedVersions, Version};
use crate::platform;
use crate::Result;
use crate::{apps, output};
use std::process::ExitCode;

use super::run::load_or_install;

pub fn which(app_name: &AppName, version: Option<Version>, log: Option<String>) -> Result<ExitCode> {
    let apps = apps::all();
    let app = apps.lookup(app_name)?;
    let output = output::StdErr { category: log };
    let platform = platform::detect(&output)?;
    let versions = RequestedVersions::determine(app_name, version, &apps)?;
    for version in versions.into_iter() {
        if let Some(executable) = load_or_install(app, version, platform, &output)? {
            println!("{}", executable.0.to_string_lossy());
            return Ok(ExitCode::SUCCESS);
        }
    }
    Ok(ExitCode::FAILURE)
}
