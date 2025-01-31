use super::run::load_or_install;
use crate::applications::ApplicationName;
use crate::configuration::{self, RequestedVersions, Version};
use crate::prelude::*;
use crate::yard::Yard;
use crate::{applications, logging, platform, yard};
use std::process::ExitCode;

pub(crate) fn available(args: &Args) -> Result<ExitCode> {
  let apps = applications::all();
  let app = apps.lookup(&args.app_name)?;
  let log = logging::new(args.verbose);
  let platform = platform::detect(log)?;
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  let config_file = configuration::File::load(&apps)?;
  let versions = RequestedVersions::determine(&args.app_name, args.version.as_ref(), &config_file)?;
  for version in versions {
    if load_or_install(app, &version, platform, args.optional, &yard, &config_file, log)?.is_some() {
      return Ok(ExitCode::SUCCESS);
    }
  }
  Ok(ExitCode::FAILURE)
}

#[derive(Debug, PartialEq)]
pub(crate) struct Args {
  pub(crate) app_name: ApplicationName,
  pub(crate) optional: bool,
  pub(crate) version: Option<Version>,
  pub(crate) verbose: bool,
}
