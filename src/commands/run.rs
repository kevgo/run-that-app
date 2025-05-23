use crate::applications::{AnalyzeResult, AppDefinition, ApplicationName, Apps};
use crate::configuration::{self, AppVersions, RequestedVersion, RequestedVersions, Version};
use crate::executables::{ExecutableCall, ExecutableCallDefinition};
use crate::filesystem::find_global_install;
use crate::installation::{self, Outcome};
use crate::logging::{self, Event, Log};
use crate::platform::{self, Platform};
use crate::prelude::*;
use crate::yard::Yard;
use crate::{applications, subshell, yard};
use std::process::ExitCode;

pub(crate) fn run(args: Args) -> Result<ExitCode> {
  let apps = applications::all();
  let app_to_run = apps.lookup(&args.app_name)?;
  let log = logging::new(args.verbose);
  let platform = platform::detect(log)?;
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  let config_file = configuration::File::load(&apps)?;
  let include_app_versions = config_file.lookup_many(args.include_apps);
  let include_apps = load_or_install_apps(include_app_versions, &apps, platform, args.optional, &yard, &config_file, args.from_source, log)?;
  let requested_versions = RequestedVersions::determine(&args.app_name, args.version.as_ref(), &config_file)?;
  let Some(executable_call) = load_or_install_app(
    app_to_run,
    requested_versions,
    platform,
    args.optional,
    &yard,
    &config_file,
    args.from_source,
    log,
  )?
  else {
    if args.optional {
      return Ok(ExitCode::SUCCESS);
    }
    return Err(UserError::UnsupportedPlatform);
  };
  if args.error_on_output {
    let (executable, args) = executable_call.with_args(args.app_args);
    subshell::detect_output(&executable, &args, &include_apps)
  } else {
    let (executable, args) = executable_call.with_args(args.app_args);
    subshell::stream_output(&executable, &args, &include_apps)
  }
}

/// data needed to run an executable
#[derive(Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct Args {
  /// name of the app to execute
  pub(crate) app_name: ApplicationName,

  /// possible versions of the app to execute
  pub(crate) version: Option<Version>,

  /// arguments to call the app with
  #[allow(clippy::struct_field_names)]
  pub(crate) app_args: Vec<String>,

  /// if true, any output produced by the app is equivalent to an exit code > 0
  pub(crate) error_on_output: bool,

  /// if true, install only from source
  pub(crate) from_source: bool,

  /// other applications to include into the PATH
  pub(crate) include_apps: Vec<ApplicationName>,

  /// whether it's okay to not run the app if it cannot be installed
  pub(crate) optional: bool,

  pub(crate) verbose: bool,
}

fn load_or_install_apps(
  app_versions: Vec<AppVersions>,
  apps: &Apps,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  config_file: &configuration::File,
  from_source: bool,
  log: Log,
) -> Result<Vec<ExecutableCall>> {
  let mut result = vec![];
  for app_version in app_versions {
    let app = apps.lookup(app_version.app_name)?;
    if let Some(executable_call) = load_or_install_app(app, app_version.versions, platform, optional, yard, config_file, from_source, log)? {
      result.push(executable_call);
    }
  }
  Ok(result)
}

pub(crate) fn load_or_install_app(
  app_definition: &dyn AppDefinition,
  requested_versions: RequestedVersions,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  config_file: &configuration::File,
  from_source: bool,
  log: Log,
) -> Result<Option<ExecutableCall>> {
  for requested_version in requested_versions {
    if let Some(executable_call) = load_or_install(app_definition, &requested_version, platform, optional, yard, config_file, from_source, log)? {
      return Ok(Some(executable_call));
    }
  }
  Ok(None)
}

fn load_or_install(
  app_definition: &dyn AppDefinition,
  requested_version: &RequestedVersion,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  config_file: &configuration::File,
  from_source: bool,
  log: Log,
) -> Result<Option<ExecutableCall>> {
  match requested_version {
    RequestedVersion::Path(version) => {
      if let Some(executable_call_def) = load_from_path(app_definition, version, platform, log)? {
        if let Some(app_folder) = executable_call_def.executable.clone().as_path().parent() {
          if let Some(executable_call) = executable_call_def.into_executable_call(app_folder) {
            return Ok(Some(executable_call));
          }
        }
      }
      Ok(None)
    }
    RequestedVersion::Yard(version) => load_or_install_from_yard(app_definition, version, platform, optional, yard, config_file, from_source, log),
  }
}

// finds the app in the PATH and verifies it has the correct version
fn load_from_path(app_to_run: &dyn AppDefinition, range: &semver::VersionReq, platform: Platform, log: Log) -> Result<Option<ExecutableCallDefinition>> {
  let (app_to_install, executable_name, executable_args) = app_to_run.carrier(&Version::from(""), platform);
  let executable_filename = executable_name.platform_path(platform.os);
  let Some(executable) = find_global_install(&executable_filename, log) else {
    log(Event::GlobalInstallNotFound);
    return Ok(None);
  };
  match app_to_install.analyze_executable(&executable, log)? {
    AnalyzeResult::NotIdentified { output: _ } => {
      log(Event::GlobalInstallNotIdentified);
      Ok(None)
    }
    AnalyzeResult::IdentifiedButUnknownVersion if range.to_string() == "*" => {
      log(Event::GlobalInstallMatchingVersion { range, version: None });
      Ok(Some(ExecutableCallDefinition {
        executable,
        args: executable_args,
      }))
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
      Ok(Some(ExecutableCallDefinition {
        executable,
        args: executable_args,
      }))
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
  app_definition: &dyn AppDefinition,
  version: &Version,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  config_file: &configuration::File,
  from_source: bool,
  log: Log,
) -> Result<Option<ExecutableCall>> {
  let (app_to_install, executable_name, executable_args) = app_definition.carrier(version, platform);
  let app_name = app_to_install.app_name();
  // try to load the app
  if let Some((executable, bin_folder)) = yard.load_executable(app_to_install.as_ref(), &executable_name, version, platform, log) {
    let app_folder = yard.app_folder(&app_name, version);
    let args = executable_args.locate(&app_folder, &bin_folder)?;
    return Ok(Some(ExecutableCall { executable, args }));
  }
  // app not installed --> check if uninstallable
  if yard.is_not_installable(&app_name, version) {
    return Ok(None);
  }
  // app not installed and installable --> try to install
  match installation::any(app_to_install.as_ref(), version, platform, optional, yard, config_file, from_source, log)? {
    Outcome::Installed => {} // we'll load it below
    Outcome::NotInstalled => {
      yard.mark_not_installable(&app_name, version)?;
      return Ok(None);
    }
  }
  // load again now that it is installed
  if let Some((executable, bin_folder)) = yard.load_executable(app_to_install.as_ref(), &executable_name, version, platform, log) {
    let app_folder = yard.app_folder(&app_name, version);
    let args = executable_args.locate(&app_folder, &bin_folder)?;
    return Ok(Some(ExecutableCall { executable, args }));
  }
  Err(UserError::CannotFindExecutable)
}
