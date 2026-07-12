use crate::applications::{ApplicationName, Apps};
use crate::configuration::{self, Version};
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::executables::{LoadOrInstallAppAndCarrierArgs, LoadOrInstallAppOutcome, load_or_install_app_and_carrier, load_or_install_apps};
use crate::yard::Yard;
use crate::{logging, platform, yard};
use std::process::ExitCode;

pub fn install(
  InstallArgs {
    app_name,
    version,
    from_source,
    include_apps,
    optional,
    verbose,
  }: InstallArgs,
  apps: &Apps,
) -> Result<ExitCode> {
  let app_to_install = apps.lookup(&app_name)?;
  let log = logging::new(verbose);
  let platform = platform::detect(log)?;
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  let config_file = configuration::File::load(apps)?;
  let ctx = RuntimeContext {
    platform,
    yard: &yard,
    config_file: &config_file,
    log,
  };
  // install the included apps
  let include_apps = apps.lookup_many(&include_apps)?;
  load_or_install_apps(include_apps, apps, optional, &ctx)?;
  // install the main app
  match load_or_install_app_and_carrier(LoadOrInstallAppAndCarrierArgs {
    app: app_to_install,
    cli_version: version.as_ref(),
    optional,
    from_source,
    ctx: &ctx,
    apps,
  })? {
    LoadOrInstallAppOutcome::Loaded { executable_call: _ } => Ok(ExitCode::SUCCESS),
    LoadOrInstallAppOutcome::NotInstallable { app: _ } if optional => Ok(ExitCode::SUCCESS),
    LoadOrInstallAppOutcome::NotInstallable { app } => Err(UserError::UnsupportedPlatform { app }),
  }
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
