use crate::applications::{ApplicationName, Apps};
use crate::configuration::{self, Version};
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::{LoadOrInstallAppOutcome, LoadOrInstallAppWithCarrierArgs, load_or_install_app_and_carrier};
use crate::yard::Yard;
use crate::{logging, platform, yard};
use std::process::ExitCode;

pub fn available(args: &AvailableArgs, apps: &Apps) -> Result<ExitCode> {
  let app = apps.lookup(&args.app_name)?;
  let log = logging::new(args.verbose);
  let platform = platform::detect(log)?;
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  let config_file = configuration::File::load(apps)?;
  let ctx = RuntimeContext {
    platform,
    yard: &yard,
    config_file: &config_file,
    log,
  };
  let outcome = load_or_install_app_and_carrier(LoadOrInstallAppWithCarrierArgs {
    app,
    cli_version: None,
    optional: args.optional,
    from_source: false,
    ctx: &ctx,
    apps,
  })?;
  match outcome {
    LoadOrInstallAppOutcome::Loaded { executable_call: _ } => Ok(ExitCode::SUCCESS),
    LoadOrInstallAppOutcome::NotInstallable { app: _ } => Ok(ExitCode::FAILURE),
  }
}

/// named arguments for the [`available`] command
#[derive(Debug, PartialEq)]
pub struct AvailableArgs {
  pub app_name: ApplicationName,
  pub optional: bool,
  pub version: Option<Version>,
  pub verbose: bool,
}
