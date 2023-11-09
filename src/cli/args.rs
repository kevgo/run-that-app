use super::Command;
use super::{requested_app, RequestedApp};
use crate::error::UserError;
use crate::Result;

/// all arguments that can be provided via the CLI
#[derive(Debug, PartialEq)]
pub struct Args {
    pub command: Command,
    pub log: Option<String>,
}

pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Args> {
    let _skipped_binary_name = args.next();
    let mut requested_app: Option<RequestedApp> = None;
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
        if requested_app.is_none() {
            requested_app = Some(requested_app::parse(&arg)?);
        } else {
            return Err(UserError::DuplicateRunRequest);
        }
    }
    if let Some(requested_app) = requested_app {
        Ok(Args {
            command: Command::RunApp { app: requested_app },
            log,
        })
    } else {
        Ok(Args {
            command: Command::DisplayHelp,
            log,
        })
    }
}

#[cfg(test)]
mod tests {

    mod parse {
        use crate::cli::{args, Args, Command};

        #[test]
        fn no_arguments() {
            let args = vec!["run-that-app"].into_iter().map(ToString::to_string);
            let have = args::parse(args).unwrap();
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
            let have = args::parse(args).unwrap();
            let want = Args {
                log: None,
                command: Command::DisplayVersion,
            };
            pretty::assert_eq!(have, want);
        }

        mod logging {
            use crate::cli::{args, Args, Command, RequestedApp};
            use big_s::S;

            #[test]
            fn everything() {
                let args = vec!["run-that-app", "--log", "app@2"]
                    .into_iter()
                    .map(ToString::to_string);
                let have = args::parse(args).unwrap();
                let want = Args {
                    command: Command::RunApp {
                        app: RequestedApp {
                            name: S("app"),
                            version: S("2"),
                        },
                    },
                    log: Some(S("")),
                };
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn limited() {
                let args = vec!["run-that-app", "--log=scope", "app@2"]
                    .into_iter()
                    .map(ToString::to_string);
                let have = args::parse(args).unwrap();
                let want = Args {
                    command: Command::RunApp {
                        app: RequestedApp {
                            name: S("app"),
                            version: S("2"),
                        },
                    },
                    log: Some(S("scope")),
                };
                pretty::assert_eq!(have, want);
            }
        }
    }
}
