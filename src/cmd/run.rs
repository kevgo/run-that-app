use crate::apps;
use crate::apps::{AnalyzeResult, App};
use crate::config::{AppName, RequestedVersion, RequestedVersions, Version};
use crate::filesystem::find_global_install;
use crate::logger::{self, Event, Log};
use crate::platform::{self, Platform};
use crate::prelude::*;
use crate::subshell::{self, Executable};
use crate::yard::Yard;
use crate::{install, yard};
use std::process::ExitCode;

pub fn run(args: Args) -> Result<ExitCode> {
  let apps = apps::all();
  let app = apps.lookup(&args.app)?;
  let log = logger::new(args.verbose);
  let platform = platform::detect(log)?;
  let yard = yard::load_or_create(&yard::production_location()?)?;
  let versions = RequestedVersions::determine(&args.app, args.version, &apps)?;
  for version in versions {
    if let Some(executable) = load_or_install(app, &version, platform, &yard, log)? {
      if args.error_on_output {
        return subshell::execute_check_output(&executable, &args.app_args);
      }
      return subshell::execute_stream_output(&executable, &args.app_args);
    }
  }
  if args.optional {
    Ok(ExitCode::SUCCESS)
  } else {
    Err(UserError::UnsupportedPlatform)
  }
}

/// data needed to run an executable
pub struct Args {
  /// name of the app to execute
  pub app: AppName,

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

pub fn load_or_install(app: &dyn App, version: &RequestedVersion, platform: Platform, yard: &Yard, log: Log) -> Result<Option<Executable>> {
  match version {
    RequestedVersion::Path(version) => load_from_path(app, version, platform, log),
    RequestedVersion::Yard(version) => load_or_install_from_yard(app, version, platform, yard, log),
  }
}

// checks if the app is in the PATH and has the correct version
fn load_from_path(app: &dyn App, range: &semver::VersionReq, platform: Platform, log: Log) -> Result<Option<Executable>> {
  let Some(executable) = find_global_install(&app.executable_filename(platform), log) else {
    log(Event::GlobalInstallNotFound);
    return Ok(None);
  };
  match app.analyze_executable(&executable, log)? {
    AnalyzeResult::NotIdentified { output: _ } => {
      log(Event::GlobalInstallNotIdentified);
      Ok(None)
    }
    AnalyzeResult::IdentifiedButUnknownVersion if range.to_string() == "*" => {
      log(Event::GlobalInstallMatchingVersion { range, version: None });
      Ok(Some(executable))
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
      Ok(Some(executable))
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

fn load_or_install_from_yard(app: &dyn App, version: &Version, platform: Platform, yard: &Yard, log: Log) -> Result<Option<Executable>> {
  // try to load the app
  if let Some(executable) = install::load(app.install_methods(), version, platform, yard, log) {
    return Ok(Some(executable));
  }
  // app not installed --> check if uninstallable
  if yard.is_not_installable(&app.name(), version) {
    return Ok(None);
  }
  // app not installed and installable --> try to install
  if install::any(app.install_methods(), version, platform, yard, log)? {
    return Ok(install::load(app.install_methods(), version, platform, yard, log));
  }

  // app could not be installed -> mark as uninstallable
  yard.mark_not_installable(&app.name(), version)?;
  Ok(None)
}
