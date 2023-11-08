use super::run_request;
use super::RunRequest;
use crate::error::UserError;
use crate::Result;

pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Args> {
    let _skipped_binary_name = args.next();
    let mut run_request: Option<RunRequest> = None;
    let mut log: Option<String> = None;
    for arg in args {
        if &arg == "--help" || &arg == "-h" {
            return Ok(Args {
                command: Command::DisplayHelp,
                log: None,
            });
        }
        if &arg == "--version" || &arg == "-V" {
            return Ok(Args {
                command: Command::DisplayVersion,
                log: None,
            });
        }
        if arg.starts_with('-') {
            let (key, value) = arg.split_once('=').unwrap_or((&arg, ""));
            if key == "--log" || key == "-l" {
                log = Some(value.to_string());
                continue;
            }
            return Err(UserError::UnknownCliOption(arg));
        }
        if run_request.is_none() {
            run_request = Some(run_request::parse(&arg));
        } else {
            return Err(UserError::DuplicateRunRequest);
        }
    }
    Ok(Args {
        command: Command::DisplayHelp,
        log,
    })
}

#[derive(Debug, PartialEq)]
pub struct Args {
    pub log: Option<String>,
    pub command: Command,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    RunApp { name: String, version: String },
    DisplayHelp,
    DisplayVersion,
}

#[cfg(test)]
mod tests {
    use super::Args;
    use super::Command;

    #[test]
    fn no_arguments() {
        let args = vec!["run-that-app"].into_iter().map(ToString::to_string);
        let have = super::parse(args).unwrap();
        let want = Args {
            log: None,
            command: Command::DisplayHelp,
        };
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn display_version() {
        let args = vec!["run-that-app", "-V"]
            .into_iter()
            .map(ToString::to_string);
        let have = super::parse(args).unwrap();
        let want = Args {
            log: None,
            command: Command::DisplayVersion,
        };
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn logging() {
        let args = vec!["run-that-app", "-V"]
            .into_iter()
            .map(ToString::to_string);
        let have = super::parse(args).unwrap();
        let want = Args {
            log: None,
            command: Command::DisplayVersion,
        };
        pretty::assert_eq!(have, want);
    }
}
