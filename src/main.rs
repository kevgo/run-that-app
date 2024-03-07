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
mod output;
mod platform;
mod regexp;
mod subshell;
mod yard;

use cli::Command;
use cmd::run;
use error::{Result, UserError};
use output::Output;
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

fn inner() -> Result<ExitCode> {
    let cli_args = cli::parse(std::env::args())?;
    match cli_args.command {
        Command::Available { app, version, log } => cmd::available(&app, version, log),
        Command::RunApp {
            log,
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
            log,
        }),
        Command::DisplayHelp => Ok(cmd::help()),
        Command::Setup => cmd::setup(),
        Command::Which { app, version, log } => cmd::which(&app, version, log),
        Command::Update { log } => {
            let output = output::StdErr { category: log };
            cmd::update(&output)
        }
        Command::Version => Ok(cmd::version()),
        Command::Versions { app, amount, log } => {
            let output = output::StdErr { category: log };
            cmd::versions(&app, amount, &output)
        }
    }
}
