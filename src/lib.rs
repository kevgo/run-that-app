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
//!   cwd: None,
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
use crate::context::RuntimeContext;
pub use crate::executables::CommandInfo;
use crate::executables::{load_or_install_app_and_carrier, load_or_install_apps};
use crate::yard::Yard;
use cli::Cli;
pub use configuration::Version;
#[cfg(test)]
pub use error::UserError;
use logging::Log;
use std::path::Path;
use std::process::ExitCode;

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

/// Provides a placeholder for a fully configured [`std::process::Command`] instance
/// that executes the given app with the given arguments.
/// You can run it any way you like.
/// The placeholder differs that it is able to provide information about the command to execute.
/// You can convert [`CommandInfo`] instances into actual [`std::process::Command`] instances via the `From` trait.
///
/// # Examples
///
/// ```
/// let actionlint = rta::applications::ActionLint {};
/// let cmd_info = rta::get_cmd(
///   &actionlint,
///   rta::GetCmdArgs {
///     version: Some("1.7.12".into()),
///     app_args: vec!["--help".into()],
///     from_source: false,
///     include_apps: vec![],
///     optional: false,
///     verbose: false,
///   },
///   &rta::applications::all(),
/// );
///
/// let Ok(cmd_info) = cmd_info else {
///   panic!("ran into an error: {:?}", cmd_info.err());
/// };
/// let Some(mut cmd_info) = cmd_info else {
///   panic!("actionlint is not supported on this platform");
/// };
///
/// let mut cmd = std::process::Command::from(cmd_info);
/// let exit_status = cmd.status().unwrap();
/// assert!(exit_status.success());
/// ```
pub fn get_cmd(app: &dyn AppDefinition, args: GetCmdArgs, apps: &Apps) -> Result<Option<CommandInfo>, error::UserError> {
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

  // Open question:
  // Should we install the carrier app here at the top level,
  // or have it install transparently in `load_or_install_app`?
  //
  // An argument for installing it here at the top is
  // that this keeps the implementation of `load_or_install_app` simpler.
  // If we would install the carrier in `load_or_install_app`,
  // we would have two layers of `load_or_install` calls:
  // A high-level that installs the app or the carrier,
  // and a low-level that installs only the given app.
  //
  // One could argue that the scope of `load_or_install_app` should be limited to
  // loading or installing the given app,
  // i.e. it should be low-level.
  // High-level operations like installing the carrier should be done at the top level.
  //
  // On the other hand, we need this logic several times at the top level:
  // - installing the app to run (including its carrier)
  // - installing other apps to include (including their carrier)
  //
  // So we do need separate low-level and high-level `load_or_install` functions.
  //
  // Another argument for extracting this logic into a separate function is
  // that it would create too much complexity here at the top level.
  // The top level should orchestrate activities, not perform them.

  let include_app_names = args.include_apps.iter().map(|app| app.name()).collect();
  let include_app_versions = config_file.lookup_many(include_app_names);
  let include_apps = load_or_install_apps(&include_app_versions, apps, args.optional, args.from_source, &ctx)?;

  let Some(executable_call) = load_or_install_app_and_carrier(app, args.version.as_ref(), &config_file, args.optional, args.from_source, &ctx, apps)? else {
    if args.optional {
      return Ok(None);
    }
    return Err(error::UserError::UnsupportedPlatform);
  };
  let (executable, args) = executable_call.with_args(args.app_args);
  let mut paths_to_include: Vec<&Path> = vec![&executable.parent_path()];

  for app_to_include in &include_apps {
    paths_to_include.push(app_to_include.executable.parent_path());
  }
  let env_path = subshell::path_expressions(&paths_to_include);
  let cmd_info = CommandInfo {
    executable: executable.into(),
    args,
    env_path,
  };
  Ok(Some(cmd_info))
}

/// data needed to run an executable
#[allow(clippy::struct_excessive_bools)]
pub struct GetCmdArgs<'a> {
  /// version of the app to execute
  pub version: Option<Version>,

  /// arguments to call the app with
  #[allow(clippy::struct_field_names)]
  pub app_args: Vec<String>,

  /// if true, install only from source
  pub from_source: bool,

  /// other applications to include into the PATH
  pub include_apps: Vec<&'a dyn AppDefinition>,

  /// whether it's okay to not run the app if it cannot be installed
  pub optional: bool,

  pub verbose: bool,
}
