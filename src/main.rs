mod apps;
mod cmd;
mod detect;
mod error;
mod hosting;
mod subshell;
mod ui;
mod yard;

use detect::Platform;
use error::{Result, UserError};
use std::process::ExitCode;
use ui::Command;
use ui::Output;

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
    let args = ui::parse(std::env::args())?;
    let output = ui::Output { category: args.log };
    match args.command {
        Command::RunApp { app: request } => cmd::run(request, &output)?,
        Command::DisplayHelp => cmd::help(&output),
        Command::DisplayVersion => cmd::version(&output),
    }
    Ok(())
}
