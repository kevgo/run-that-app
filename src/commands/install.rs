use crate::applications::{ApplicationName, Apps};
use crate::configuration::{self, RequestedVersions, Version};
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::yard::Yard;
use crate::{commands, logging, platform, yard};
use std::process::ExitCode;

pub fn install(args: InstallArgs, apps: &Apps) -> Result<ExitCode> {
  let app_to_install = apps.lookup(&args.app_name)?;
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
  let include_app_versions = config_file.lookup_many(args.include_apps);
  let _include_apps = commands::run::load_or_install_apps(&include_app_versions, apps, args.optional, args.from_source, &ctx)?;
  let requested_versions = RequestedVersions::determine(app_to_install, args.version.as_ref(), &config_file, log)?;
  let Some(_executable_call) = commands::run::load_or_install_app(app_to_install, &requested_versions, args.optional, args.from_source, &ctx, apps)? else {
    if args.optional {
      return Ok(ExitCode::SUCCESS);
    }
    return Err(UserError::UnsupportedPlatform);
  };
  Ok(ExitCode::SUCCESS)
}

/// named arguments for the [`install`], [`install_all`][super::install_all::install_all], and [`reinstall`][super::reinstall::reinstall] commands
#[derive(Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct InstallArgs {
  /// name of the app to install
  pub app_name: ApplicationName,

  /// possible versions of the app to install
  pub version: Option<Version>,

  /// if true, install only from source
  pub from_source: bool,

  /// other applications to include
  pub include_apps: Vec<ApplicationName>,

  /// whether it's okay to not install the app if it cannot be installed
  pub optional: bool,

  pub verbose: bool,
}
