//! run-that-app runs third-party applications without the need to install them
//!
//! # Examples
//!
//! The easiest way to run an app
//! is to provide the CLI arguments you would use
//! when executing the run-that-app binary:
//! ```
//! let args: Vec<String> = vec!["--verbose".into(), "gh@2.34.0".into(), "--version".into()];
//! let exit_code = rta::run(args.into_iter());
//! ```
//!
//! You can also use the programmatic API:
//!
//! ```
//! use std::process::ExitCode;
//! use crate::rta::applications::AppDefinition;
//!
//! // call the "gh" app at version "2.34.0" with the argument "--version"
//! let gh = rta::applications::Gh {};
//! let apps = rta::applications::all();
//!
//! let args = rta::commands::RunArgs {
//!   app_name: gh.name(),
//!   version: Some("2.34.0".into()),
//!   app_args: vec!["--version".into()],
//!   error_on_output: false,
//!   from_source: false,
//!   include_apps: vec![],
//!   optional: true,
//!   verbose: false,
//! };
//! let result = rta::commands::run(args, &apps);
//! match result {
//!   Ok(exit_code) => println!("app ran, check exit code"),
//!   Err(error) => {
//!     println!("app failed to run");
//!     error.print();
//!   }
//! }
//! ```

pub mod applications;
mod archives;
mod cli;
pub mod commands;
mod configuration;
mod context;
mod download;
pub mod error;
mod executables;
mod filesystem;
mod hosting;
mod installation;
pub mod logging;
mod platform;
mod strings;
mod subshell;
mod yard;
use crate::applications::{AppDefinition, Apps};
use crate::commands::{load_or_install_app, load_or_install_apps};
use crate::configuration::RequestedVersions;
use crate::context::RuntimeContext;
use crate::subshell::add_paths;
use crate::yard::Yard;
use cli::Cli;
pub use configuration::Version;
#[cfg(test)]
pub use error::UserError;
use logging::Log;
use std::path::Path;
use std::process::{Command, ExitCode};

/// Runs run-that-app with the given CLI arguments.
///
/// Example:
/// ```
/// let args = vec![String::from("gh@2.34.0"), String::from("--version")];
/// let exit_code = rta::run(args.into_iter());
/// ```
pub fn run(args: impl Iterator<Item = String>) -> error::Result<ExitCode> {
  let apps = applications::all();
  match cli::parse(args, &apps)? {
    Cli::Add(args) => commands::add(args, &apps),
    Cli::AppsLong => Ok(commands::applications::long(&apps)),
    Cli::AppsShort => Ok(commands::applications::short(&apps)),
    Cli::Available(args) => commands::available(&args, &apps),
    Cli::DisplayHelp => Ok(commands::help()),
    Cli::Install(args) => commands::install(args, &apps),
    Cli::InstallAll => commands::install_all(&apps),
    Cli::Reinstall(args) => commands::reinstall(args, &apps),
    Cli::RunApp(args) => commands::run(args, &apps),
    Cli::Test(mut args) => commands::test(&mut args, &apps),
    Cli::Update(args) => commands::update(&args, &apps),
    Cli::Version => Ok(commands::version()),
    Cli::Versions(args) => commands::versions(&args, &apps),
    Cli::Which(args) => commands::which(&args, &apps),
  }
}

/// Provides a fully configured [`std::process::Command`] instance
/// that executes the given app with the given arguments.
/// You can run it any way you like.
///
/// # Examples
///
/// ```
/// use std::process::ExitCode;
///
/// let actionlint = rta::applications::ActionLint {};
/// let apps = rta::applications::all();
/// let cmd = rta::get_cmd(
///   &actionlint,
///   rta::GetCmdArgs {
///     version: Some("1.7.12".into()),
///     app_args: vec!["--version".into()],
///     error_on_output: false,
///     from_source: false,
///     include_apps: vec![],
///     optional: false,
///     verbose: false,
///   },
///   &apps,
/// );
/// let mut cmd = cmd.unwrap().unwrap();
/// let exit_status = cmd.status().unwrap();
/// assert!(exit_status.success());
/// ```
#[allow(clippy::missing_panics_doc)] // all the unwraps here never happen
pub fn get_cmd(app: &dyn AppDefinition, args: GetCmdArgs, apps: &Apps) -> Result<Option<Command>, error::UserError> {
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
  let app_names = args.include_apps.iter().map(|app| app.name()).collect();
  let include_app_versions = config_file.lookup_many(app_names);
  let include_apps = load_or_install_apps(&include_app_versions, apps, args.optional, args.from_source, &ctx)?;
  let requested_versions = RequestedVersions::determine(&app.name(), args.version.as_ref(), &config_file)?;
  let Some(executable_call) = load_or_install_app(app, &requested_versions, args.optional, args.from_source, &ctx)? else {
    if args.optional {
      return Ok(None);
    }
    return Err(error::UserError::UnsupportedPlatform);
  };
  let (executable, args) = executable_call.with_args(args.app_args);
  #[allow(clippy::unwrap_used)] // there is always a parent here since this is a location inside the yard
  let mut paths_to_include: Vec<&Path> = vec![&executable.as_path().parent().unwrap()];
  let mut cmd = Command::new(&executable);
  cmd.args(&args);
  for app_to_include in &include_apps {
    #[allow(clippy::unwrap_used)] // there is always a parent here since this is a location inside the yard
    paths_to_include.push(app_to_include.executable.as_path().parent().unwrap());
  }
  add_paths(&mut cmd, &paths_to_include);
  Ok(Some(cmd))
}

/// data needed to run an executable
#[derive(Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct GetCmdArgs {
  /// possible versions of the app to execute
  pub version: Option<Version>,

  /// arguments to call the app with
  #[allow(clippy::struct_field_names)]
  pub app_args: Vec<String>,

  /// if true, any output produced by the app is equivalent to an exit code > 0
  pub error_on_output: bool,

  /// if true, install only from source
  pub from_source: bool,

  /// other applications to include into the PATH
  pub include_apps: Vec<Box<dyn AppDefinition>>,

  /// whether it's okay to not run the app if it cannot be installed
  pub optional: bool,

  pub verbose: bool,
}
