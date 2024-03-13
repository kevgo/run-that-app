use crate::apps;
use crate::config::{AppName, RequestedVersions, Version};
use crate::logger;
use crate::platform;
use crate::Result;
use std::process::ExitCode;

use super::run::load_or_install;

pub fn which(app_name: &AppName, version: Option<Version>, verbose: bool) -> Result<ExitCode> {
    let apps = apps::all();
    let app = apps.lookup(app_name)?;
    let log = logger::new(verbose);
    let platform = platform::detect(log)?;
    let versions = RequestedVersions::determine(app_name, version, &apps)?;
    for version in versions {
        if let Some(executable) = load_or_install(app, &version, platform, log)? {
            println!("{}", executable.0.to_string_lossy());
            return Ok(ExitCode::SUCCESS);
        }
    }
    Ok(ExitCode::FAILURE)
}
