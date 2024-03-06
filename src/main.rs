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
use config::RequestedVersions;
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
        Command::Available { app, version, log } => {
            let output = output::StdErr { category: log };
            let versions = RequestedVersions::determine(&app, version)?;
            cmd::available(&app, &versions, &output)
        }
        Command::RunApp {
            log,
            app,
            version,
            app_args,
            error_on_output,
            optional,
        } => {
            let output = output::StdErr { category: log };
            let versions = RequestedVersions::determine(&app, version)?;
            cmd::run(&run::Args {
                app,
                versions,
                app_args,
                error_on_output,
                optional,
                output: &output,
            })
        }
        Command::DisplayHelp => Ok(cmd::help()),
        Command::Setup => cmd::setup(),
        Command::Which { app, version, log } => {
            let output = output::StdErr { category: log };
            let versions = RequestedVersions::determine(&app, version)?;
            cmd::which(&app, &versions, &output)
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
