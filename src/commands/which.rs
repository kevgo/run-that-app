use crate::applications::{ApplicationName, Apps};
use crate::configuration::{self, Version};
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::{LoadOrInstallAppWithCarrierOutcome, load_or_install_app_and_carrier};
use crate::yard::Yard;
use crate::{logging, platform, yard};
use std::process::ExitCode;

pub fn which(args: &WhichArgs, apps: &Apps) -> Result<ExitCode> {
  let app = apps.lookup(&args.app_name)?;
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
  match load_or_install_app_and_carrier(app, args.version.as_ref(), &config_file, args.optional, false, &ctx, apps)? {
    LoadOrInstallAppWithCarrierOutcome::Loaded { executable_call } => {
      println!("{executable_call}");
      Ok(ExitCode::SUCCESS)
    }
    LoadOrInstallAppWithCarrierOutcome::NotInstallable { app: _ } => Ok(ExitCode::FAILURE),
  }
}

#[derive(Debug, PartialEq)]
pub struct WhichArgs {
  pub app_name: ApplicationName,
  pub optional: bool,
  pub version: Option<Version>,
  pub verbose: bool,
}
