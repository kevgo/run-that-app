use super::run::load_or_install;
use crate::config::{self, AppName, RequestedVersions, Version};
use crate::prelude::*;
use crate::{apps, logger, platform, yard};
use std::process::ExitCode;

pub fn available(args: &Args) -> Result<ExitCode> {
  let apps = apps::all();
  let app = apps.lookup(&args.app_name)?;
  let log = logger::new(args.verbose);
  let platform = platform::detect(log)?;
  let yard = yard::load_or_create(&yard::production_location()?)?;
  let config_file = config::File::load(&apps)?;
  let versions = RequestedVersions::determine(&args.app_name, args.version.as_ref(), &config_file)?;
  for version in versions {
    if load_or_install(app, &version, platform, &yard, &config_file, log)?.is_some() {
      return Ok(ExitCode::SUCCESS);
    }
  }
  Ok(ExitCode::FAILURE)
}

#[derive(Debug, PartialEq)]
pub struct Args {
  pub app_name: AppName,
  pub version: Option<Version>,
  pub verbose: bool,
}
