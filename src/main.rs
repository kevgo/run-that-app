mod applications;
mod archives;
mod cli;
mod commands;
mod configuration;
mod download;
mod error;
mod executables;
mod filesystem;
mod hosting;
mod installation;
mod logging;
mod platform;
mod prelude;
mod regexp;
mod subshell;
mod yard;

use cli::Command;
#[cfg(test)]
pub(crate) use error::UserError;
use logging::Log;
use std::process::ExitCode;

fn main() -> ExitCode {
  match inner() {
    Ok(exitcode) => exitcode,
    Err(err) => {
      err.print();
      ExitCode::FAILURE
    }
  }
}

fn inner() -> prelude::Result<ExitCode> {
  match cli::parse(std::env::args(), &applications::all())? {
    Command::Add(args) => commands::add(args),
    Command::AppsLong => Ok(commands::applications::long()),
    Command::AppsShort => Ok(commands::applications::short()),
    Command::Available(args) => commands::available(&args),
    Command::DisplayHelp => Ok(commands::help()),
    Command::RunApp(args) => commands::run(args),
    Command::Test(mut args) => commands::test(&mut args),
    Command::Update(args) => commands::update(&args),
    Command::Which(args) => commands::which(&args),
    Command::Version => Ok(commands::version()),
    Command::Versions(args) => commands::versions(&args),
  }
}
