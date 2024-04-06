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
use cmd::run;
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
    Command::Available { app, version, verbose } => cmd::available(&app, version, verbose),
    Command::RunApp {
      verbose,
      app,
      version,
      app_args,
      error_on_output,
      optional,
    } => cmd::run(run::Args {
      app,
      version,
      app_args,
      error_on_output,
      optional,
      verbose,
    }),
    Command::DisplayHelp => Ok(cmd::help()),
    Command::Setup => cmd::setup(),
    Command::Test { app, verbose } => cmd::test(app, verbose),
    Command::Which { app, version, verbose } => cmd::which(&app, version, verbose),
    Command::Update { verbose } => cmd::update(verbose),
    Command::Version => Ok(cmd::version()),
    Command::Versions { app, amount, verbose } => cmd::versions(&app, amount, verbose),
  }
}
