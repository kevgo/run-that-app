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
use cli::Cli;
pub use configuration::Version;
#[cfg(test)]
pub use error::UserError;
use logging::Log;
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
