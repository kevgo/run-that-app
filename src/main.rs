mod cli;
mod cmd;
mod error;

use cli::Command;
use cli::Logger;
use error::Result;
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
    let logger = cli::Logger { category: args.log };
    match args.command {
        Command::RunApp { name, version } => cmd::run(&logger)?,
        Command::DisplayHelp => {}
        Command::DisplayVersion => todo!(),
    }
    Ok(())
}
