mod apps;
mod archives;
mod cli;
mod cmd;
mod config;
mod download;
mod error;
mod filesystem;
mod hosting;
mod install;
mod logger;
mod platform;
mod prelude;
mod regexp;
mod subshell;
mod yard;

use cli::Command;
use logger::Log;
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
  let cli_args = cli::parse(std::env::args())?;
  match cli_args.command {
    Command::AppsLong => Ok(cmd::apps::long()),
    Command::AppsShort => Ok(cmd::apps::short()),
    Command::Available(args) => cmd::available(args),
    Command::RunApp(args) => cmd::run(args),
    Command::DisplayHelp => Ok(cmd::help()),
    Command::Setup => cmd::setup(),
    Command::Test(mut args) => cmd::test(&mut args),
    Command::Which(args) => cmd::which(args),
    Command::Update(args) => cmd::update(&args),
    Command::Version => Ok(cmd::version()),
    Command::Versions(args) => cmd::versions(&args),
  }
}
