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
mod subshell;
mod yard;

use cli::Command;
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
        Command::Available { app, include_path, log } => {
            let output = output::StdErr { category: log };
            cmd::available(app, include_path, &output)
        }
        Command::RunApp { run_args, log } => {
            let output = output::StdErr { category: log };
            cmd::run(run_args, &output)
        }
        Command::DisplayHelp => Ok(cmd::help()),
        Command::Setup => cmd::setup(),
        Command::Which { app, include_path, log } => {
            let output = output::StdErr { category: log };
            cmd::which(app, include_path, &output)
        }
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
