mod apps;
mod archives;
mod cli;
mod cmd;
mod detect;
mod download;
mod error;
mod filesystem;
mod hosting;
mod output;
mod subshell;
mod yard;

use cli::Command;
use error::{Result, UserError};
use output::Output;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            err.print();
            ExitCode::FAILURE
        }
    }
}

fn inner() -> Result<()> {
    let args = cli::parse(std::env::args())?;
    let output = output::ConsoleOutput { category: args.log };
    match args.command {
        Command::RunApp { app: request } => cmd::run(request, &output)?,
        Command::DisplayHelp => cmd::help(&output),
        Command::DisplayVersion => cmd::version(&output),
    }
    Ok(())
}
