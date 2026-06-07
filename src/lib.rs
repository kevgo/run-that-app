//! run-that-app runs third-party applications without the need to install them
//!
//! Example:
//! ```
//! let args: Vec<String> = vec!["gh@2.34.0".into(), "--version".into()];
//! let exit_code = rta::run(args.into_iter());
//! ```
//!
//! Example:
//! ```
//! use rta::Version;
//!
//! // find the "gh" app programmatically
//! let apps = rta::applications::all();
//! let gh = apps.lookup("gh").unwrap();
//!
//! // call the "gh" app at version "2.34.0" with the argument "--version"
//! let args = rta::commands::RunArgs {
//!   app_name: gh.name(),
//!   version: Some(Version::from("2.34.0")),
//!   app_args: vec![String::from("--version")],
//!   error_on_output: false,
//!   from_source: false,
//!   include_apps: vec![],
//!   optional: true,
//!   verbose: false,
//! };
//! let _ = rta::commands::run(args, &apps);
//! ```

pub mod applications;
mod archives;
mod cli;
pub mod commands;
mod configuration;
mod context;
mod download;
mod error;
mod executables;
mod filesystem;
mod hosting;
mod installation;
pub mod logging;
mod platform;
mod strings;
mod subshell;
mod yard;
use cli::Command;
pub use configuration::Version;
#[cfg(test)]
pub use error::UserError;
use logging::Log;
use std::process::ExitCode;

/// Runs run-that-app with the given CLI arguments.
///
/// Example:
/// ```
/// let args = vec![String::from("--version")].into_iter();
/// let exit_code = rta::run(args);
/// ```
pub fn run(args: impl Iterator<Item = String>) -> error::Result<ExitCode> {
  let apps = applications::all();
  match cli::parse(args, &apps)? {
    Command::Add(args) => commands::add(args, &apps),
    Command::AppsLong => Ok(commands::applications::long(&apps)),
    Command::AppsShort => Ok(commands::applications::short(&apps)),
    Command::Available(args) => commands::available(&args, &apps),
    Command::DisplayHelp => Ok(commands::help()),
    Command::Install(args) => commands::install(args, &apps),
    Command::InstallAll => commands::install_all(&apps),
    Command::Reinstall(args) => commands::reinstall(args, &apps),
    Command::RunApp(args) => commands::run(args, &apps),
    Command::Test(mut args) => commands::test(&mut args, &apps),
    Command::Update(args) => commands::update(&args, &apps),
    Command::Version => Ok(commands::version()),
    Command::Versions(args) => commands::versions(&args, &apps),
    Command::Which(args) => commands::which(&args, &apps),
  }
}
