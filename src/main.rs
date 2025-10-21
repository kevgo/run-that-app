mod applications;
mod archives;
mod cli;
mod commands;
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

fn inner() -> error::Result<ExitCode> {
  let apps = applications::all();
  match cli::parse(std::env::args(), &apps)? {
    Command::Add(args) => commands::add(args, &apps),
    Command::AppsLong => Ok(commands::applications::long(&apps)),
    Command::AppsShort => Ok(commands::applications::short(&apps)),
    Command::Available(args) => commands::available(&args, &apps),
    Command::Concurrent(args) => commands::concurrent(args, &apps),
    Command::DisplayHelp => Ok(commands::help()),
    Command::Install(args) => commands::install(args, &apps),
    Command::RunApp(args) => commands::run(args, &apps),
    Command::Test(mut args) => commands::test(&mut args, &apps),
    Command::Update(args) => commands::update(&args, &apps),
    Command::Version => Ok(commands::version()),
    Command::Versions(args) => commands::versions(&args, &apps),
    Command::Which(args) => commands::which(&args, &apps),
  }
}
