use crate::applications::{ApplicationName, Apps};
use crate::configuration::{self, RequestedVersions, Version};
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::yard::Yard;
use crate::{commands, logging, platform, yard};
use std::process::ExitCode;

pub(crate) fn install(args: Args, apps: &Apps) -> Result<ExitCode> {
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
  let _ = yard.delete_app_folder(&args.app_name);
  let _include_apps = commands::run::load_or_install_apps(include_app_versions, apps, args.optional, args.from_source, &ctx)?;
  let requested_versions = RequestedVersions::determine(&args.app_name, args.version.as_ref(), &config_file)?;
  let Some(_executable_call) = commands::run::load_or_install_app(app_to_install, requested_versions, args.optional, args.from_source, &ctx)? else {
    if args.optional {
      return Ok(ExitCode::SUCCESS);
    }
    return Err(UserError::UnsupportedPlatform);
  };
  Ok(ExitCode::SUCCESS)
}

/// data needed to install an executable
#[derive(Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct Args {
  /// name of the app to install
  pub(crate) app_name: ApplicationName,

  /// possible versions of the app to install
  pub(crate) version: Option<Version>,

  /// if true, install only from source
  pub(crate) from_source: bool,

  /// other applications to include
  pub(crate) include_apps: Vec<ApplicationName>,

  /// whether it's okay to not install the app if it cannot be installed
  pub(crate) optional: bool,

  pub(crate) verbose: bool,
}
