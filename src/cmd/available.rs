use super::run::load_or_install;
use crate::apps;
use crate::config::AppName;
use crate::config::RequestedVersions;
use crate::config::Version;
use crate::logger;
use crate::platform;
use crate::prelude::*;
use crate::yard;
use std::process::ExitCode;

pub fn available(app_name: &AppName, version: Option<Version>, verbose: bool) -> Result<ExitCode> {
  let apps = apps::all();
  let app = apps.lookup(app_name)?;
  let log = logger::new(verbose);
  let platform = platform::detect(log)?;
  let yard = yard::load_or_create(&yard::production_location()?)?;
  let versions = RequestedVersions::determine(app_name, version, &apps)?;
  for version in versions {
    if load_or_install(app, &version, platform, &yard, log)?.is_some() {
      return Ok(ExitCode::SUCCESS);
    }
  }
  Ok(ExitCode::FAILURE)
}
