mod cli;
mod cmd;
mod error;

use cli::Command;
use cli::Output;
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
    let output = cli::Output { category: args.log };
    match args.command {
        Command::RunApp { name, version } => cmd::run(name, version, &output)?,
        Command::DisplayHelp => cmd::help(),
        Command::DisplayVersion => cmd::version(&output),
    }
    Ok(())
}
