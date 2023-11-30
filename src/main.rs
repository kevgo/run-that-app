mod apps;
mod archives;
mod cli;
mod cmd;
mod config;
mod download;
mod error;
mod filesystem;
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
        Command::RunApp {
            app,
            args,
            include_global,
            optional,
            log,
        } => {
            let output = output::StdErr { category: log };
            cmd::run(app, args, include_global, optional, &output)
        }
        Command::DisplayHelp => Ok(cmd::help()),
        Command::DisplayPath { app, include_global, log } => {
            let output = output::StdErr { category: log };
            Ok(cmd::display_path(app, include_global, &output)?)
        }
        Command::DisplayVersion => Ok(cmd::version()),
    }
}
