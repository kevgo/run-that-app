use super::run::load_or_install;
use crate::config::{AppName, RequestedVersions, Version};
use crate::prelude::*;
use crate::{apps, logger, platform, yard};
use std::process::ExitCode;

pub fn available(args: Args) -> Result<ExitCode> {
  let apps = apps::all();
  let app = apps.lookup(&args.app_name)?;
  let log = logger::new(args.verbose);
  let platform = platform::detect(log)?;
  let yard = yard::load_or_create(&yard::production_location()?)?;
  let versions = RequestedVersions::determine(&args.app_name, args.version, &apps)?;
  for version in versions {
    if load_or_install(app, &version, platform, &yard, log)?.is_some() {
      return Ok(ExitCode::SUCCESS);
    }
  }
  Ok(ExitCode::FAILURE)
}

#[derive(Debug, PartialEq)]
pub struct Args {
  app_name: AppName,
  version: Option<Version>,
  verbose: bool,
}
