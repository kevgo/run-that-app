mod cli;
mod cmd;
mod error;
mod platform;

use cli::Command;
use cli::Output;
use error::{Result, UserError};
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
    let output = cli::Output { category: args.log };
    match args.command {
        Command::RunApp { request } => cmd::run(request, &output)?,
        Command::DisplayHelp => cmd::help(&output),
        Command::DisplayVersion => cmd::version(&output),
    }
    Ok(())
}
