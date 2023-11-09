mod apps;
mod archives;
mod cmd;
mod detect;
mod download;
mod error;
mod hosting;
mod subshell;
mod ui;
mod yard;

use error::{Result, UserError};
use std::process::ExitCode;
use ui::Command;

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
    let args = ui::cli_args::parse(std::env::args())?;
    let output = ui::ConsoleOutput { category: args.log };
    match args.command {
        Command::RunApp { app: request } => cmd::run(request, &output)?,
        Command::DisplayHelp => cmd::help(&output),
        Command::DisplayVersion => cmd::version(&output),
    }
    Ok(())
}
