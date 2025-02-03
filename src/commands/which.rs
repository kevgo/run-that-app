use super::run::load_or_install_app;
use crate::applications::ApplicationName;
use crate::configuration::{self, RequestedVersions, Version};
use crate::prelude::*;
use crate::yard::Yard;
use crate::{applications, logging, platform, yard};
use std::process::ExitCode;

pub(crate) fn which(args: &Args) -> Result<ExitCode> {
  let apps = applications::all();
  let app = apps.lookup(&args.app_name)?;
  let log = logging::new(args.verbose);
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  let platform = platform::detect(log)?;
  let config_file = configuration::File::load(&apps)?;
  let versions = RequestedVersions::determine(&args.app_name, args.version.as_ref(), &config_file)?;
  if let Some(executable) = load_or_install_app(app, versions, platform, args.optional, &yard, &config_file, log)? {
    println!("{executable}");
    return Ok(ExitCode::SUCCESS);
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
