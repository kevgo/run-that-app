use crate::applications::{AnalyzeResult, App};
use crate::configuration::{self, ApplicationName, RequestedVersion, RequestedVersions, Version};
use crate::filesystem::find_global_install;
use crate::installation::Outcome;
use crate::logging::{self, Event, Log};
use crate::platform::{self, Platform};
use crate::prelude::*;
use crate::run::{self, ExecutableCall};
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
    if let Some(executable_call) = load_or_install(app, &requested_version, platform, args.optional, &yard, &config_file, log)? {
      println!("executable call: {executable_call}");
      if args.error_on_output {
        return run::check_output(&executable_call, &args.app_args);
      }
      return run::stream_output(&executable_call, &args.app_args);
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
    RequestedVersion::Path(version) => load_from_path(app, version, platform, log),
    RequestedVersion::Yard(version) => load_or_install_from_yard(app, version, platform, optional, yard, config_file, log),
  }
}

// checks if the app is in the PATH and has the correct version
fn load_from_path(app: &dyn App, range: &semver::VersionReq, platform: Platform, log: Log) -> Result<Option<ExecutableCall>> {
  let (app, executable_name, executable_args) = app.executable_definition(&Version::from(""), platform);
  let Some(executable_path) = find_global_install(&executable_name.platform_path(platform.os), log) else {
    log(Event::GlobalInstallNotFound);
    return Ok(None);
  };
  #[allow(clippy::unwrap_used)] // executable paths always have a parent
  let app_folder = executable_path.as_path().parent().unwrap();
  match app.analyze_executable(&executable_path, log)? {
    AnalyzeResult::NotIdentified { output: _ } => {
      log(Event::GlobalInstallNotIdentified);
      Ok(None)
    }
    AnalyzeResult::IdentifiedButUnknownVersion if range.to_string() == "*" => {
      log(Event::GlobalInstallMatchingVersion { range, version: None });
      let args = executable_args.make_absolute(app_folder);
      Ok(Some(ExecutableCall { executable_path, args }))
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
      let args = executable_args.make_absolute(app_folder);
      Ok(Some(ExecutableCall { executable_path, args }))
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
  let (app, executable_name_unix, args) = app.executable_definition(version, platform);
  println!("222222222222222222222 {args}");
  let executable_name = executable_name_unix.platform_path(platform.os);
  let app_folder = yard.app_folder(&app.name(), version);
  // try to load the app
  if let Some((executable_path, bin_folder)) = yard.load_executable(app.as_ref(), &executable_name, version, platform, log) {
    let args = args.make_absolute(&app_folder);
    println!("333333333333333333 {}", args.join(" "));
    return Ok(Some(ExecutableCall { executable_path, args }));
  }
  // app not installed --> check if uninstallable
  if yard.is_not_installable(&app.name(), version) {
    return Ok(None);
  }
  // app not installed and installable --> try to install
  match installation::any(app.as_ref(), version, platform, optional, yard, config_file, log)? {
    Outcome::Installed => {
      if let Some((executable_path, bin_folder)) = yard.load_executable(app.as_ref(), &executable_name, version, platform, log) {
        let args = args.make_absolute(bin_folder);
        Ok(Some(ExecutableCall { executable_path, args }))
      } else {
        Ok(None)
      }
    }
    Outcome::NotInstalled => {
      yard.mark_not_installable(&app.name(), version)?;
      Ok(None)
    }
  }
}
