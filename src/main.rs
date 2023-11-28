mod apps;
mod archives;
mod cli;
mod cmd;
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
    let args = cli::parse(std::env::args())?;
    let output = output::StdErr { category: args.log };
    match args.command {
        Command::RunApp {
            app,
            args,
            include_global,
        } => cmd::run(&app, args, include_global, &output),
        Command::DisplayHelp => Ok(cmd::help(&output)),
        Command::DisplayVersion => Ok(cmd::version(&output)),
    }
}
