mod archives;
mod cli;
mod configuration;
mod context;
mod download;
mod error;
mod executables;
mod filesystem;
mod hosting;
mod installation;
mod logging;
mod platform;
mod strings;
mod subshell;
mod yard;

pub mod applications;
pub mod commands;

use cli::Command;
#[cfg(test)]
pub use error::UserError;
use logging::Log;
use std::process::ExitCode;

/// Runs run-that-app with the given CLI arguments.
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
