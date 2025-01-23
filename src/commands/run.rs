use crate::applications::{AnalyzeResult, App};
use crate::configuration::{self, ApplicationName, RequestedVersion, RequestedVersions, Version};
use crate::filesystem::find_global_install;
use crate::installation::Outcome;
use crate::logging::{self, Event, Log};
use crate::platform::{self, Platform};
use crate::prelude::*;
use crate::run::{self, ExecutableArgs, ExecutableCall, ExecutablePath};
use crate::yard::Yard;
use crate::{applications, installation, yard};
use std::process::ExitCode;

pub fn run(args: &Args) -> Result<ExitCode> {
  let apps = applications::all();
  let app = apps.lookup(&args.app_name)?;
  let log = logging::new(args.verbose);
  let platform = platform::detect(log)?;
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  let config_file = configuration::File::load(&apps)?;
  let requested_versions = RequestedVersions::determine(&args.app_name, args.version.as_ref(), &config_file)?;
  for requested_version in requested_versions {
    if let Some(executable_path) = load_or_install(app, &requested_version, platform, args.optional, &yard, &config_file, log)? {
      if args.error_on_output {
        return run::check_output(&executable_path, &args.app_args);
      }
      return run::stream_output(&executable_path, &args.app_args);
    }
  }
  if args.optional {
    Ok(ExitCode::SUCCESS)
  } else {
    Err(UserError::UnsupportedPlatform)
  }
}

/// data needed to run an executable
#[derive(Debug, PartialEq)]
pub struct Args {
  /// name of the app to execute
  pub app_name: ApplicationName,

  /// possible versions of the app to execute
  pub version: Option<Version>,

  /// arguments to call the app with
  #[allow(clippy::struct_field_names)]
  pub app_args: Vec<String>,

  /// if true, any output produced by the app is equivalent to an exit code > 0
  pub error_on_output: bool,

  /// whether it's okay to not run the app if it cannot be installed
  pub optional: bool,

  pub verbose: bool,
}

pub fn load_or_install(
  app: &dyn App,
  requested_version: &RequestedVersion,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  config_file: &configuration::File,
  log: Log,
) -> Result<Option<ExecutableCall>> {
  match requested_version {
    RequestedVersion::Path(version) => {
      if let Some(executable_path) = load_from_path(app, version, platform, log)? {
        let args = match app.run_method(&Version::from(""), platform) {
          run::Method::ThisApp { install_methods: _ } | run::Method::OtherAppOtherExecutable { app: _, executable_name: _ } => ExecutableArgs::None,
          run::Method::OtherAppDefaultExecutable { app: _, args } => args,
        };
        Ok(Some(ExecutableCall { executable_path, args }))
      } else {
        Ok(None)
      }
    }
    RequestedVersion::Yard(version) => load_or_install_from_yard(app, version, platform, optional, yard, config_file, log),
  }
}

// checks if the app is in the PATH and has the correct version
fn load_from_path(app: &dyn App, range: &semver::VersionReq, platform: Platform, log: Log) -> Result<Option<ExecutablePath>> {
  let Some(executable_path) = find_global_install(&app.default_executable_filename().platform_path(platform.os), log) else {
    log(Event::GlobalInstallNotFound);
    return Ok(None);
  };
  match app.analyze_executable(&executable_path, log)? {
    AnalyzeResult::NotIdentified { output: _ } => {
      log(Event::GlobalInstallNotIdentified);
      Ok(None)
    }
    AnalyzeResult::IdentifiedButUnknownVersion if range.to_string() == "*" => {
      log(Event::GlobalInstallMatchingVersion { range, version: None });
      Ok(Some(executable_path))
    }
    AnalyzeResult::IdentifiedButUnknownVersion => {
      log(Event::GlobalInstallMismatchingVersion { range, version: None });
      Ok(None)
    }
    AnalyzeResult::IdentifiedWithVersion(version) if range.matches(&version.semver()?) => {
      log(Event::GlobalInstallMatchingVersion {
        range,
        version: Some(&version),
      });
      Ok(Some(executable_path))
    }
    AnalyzeResult::IdentifiedWithVersion(version) => {
      log(Event::GlobalInstallMismatchingVersion {
        range,
        version: Some(&version),
      });
      Ok(None)
    }
  }
}

fn load_or_install_from_yard(
  app: &dyn App,
  version: &Version,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  config_file: &configuration::File,
  log: Log,
) -> Result<Option<ExecutableCall>> {
  let (app, executable_name, args) = app.carrier(version, platform);
  // try to load the app
  if let Some(executable_path) = yard.load_executable(app.as_ref(), &executable_name, version, platform, log) {
    return Ok(Some(ExecutableCall { executable_path, args }));
  }
  // app not installed --> check if uninstallable
  if yard.is_not_installable(&app.name(), version) {
    return Ok(None);
  }
  // app not installed and installable --> try to install
  match installation::any(app.as_ref(), version, platform, optional, yard, config_file, log)? {
    Outcome::Installed => {
      if let Some(executable_path) = yard.load_executable(app.as_ref(), &executable_name, version, platform, log) {
        Ok(Some(ExecutableCall { executable_path, args }))
      } else {
        Err(UserError::CannotFindExecutable {
          app: app.name().to_string(),
          executable_name: executable_name.to_string(),
        })
      }
    }
    Outcome::NotInstalled => {
      yard.mark_not_installable(&app.name(), version)?;
      Ok(None)
    }
  }
}
