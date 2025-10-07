use super::run::load_or_install_app;
use crate::applications::{ApplicationName, Apps};
use crate::configuration::{self, RequestedVersions, Version};
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::yard::Yard;
use crate::{logging, platform, yard};
use std::process::ExitCode;

pub(crate) fn which(args: &Args, apps: &Apps) -> Result<ExitCode> {
  let app = apps.lookup(args.app_name.as_ref())?;
  let log = logging::new(args.verbose);
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  let platform = platform::detect(log)?;
  let config_file = configuration::File::load(apps)?;
  let ctx = RuntimeContext {
    platform,
    yard: &yard,
    config_file: &config_file,
    log,
  };
  let versions = RequestedVersions::determine(&args.app_name, args.version.as_ref(), &config_file)?;
  if let Some(executable) = load_or_install_app(app, versions, args.optional, false, &ctx)? {
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
