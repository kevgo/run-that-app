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

pub fn parse(mut cli_args: impl Iterator<Item = String>) -> Result<Args> {
    let _skipped_binary_name = cli_args.next();
    let mut requested_app: Option<RequestedApp> = None;
    let mut log: Option<String> = None;
    let mut app_args: Vec<String> = vec![];
    for arg in cli_args {
        if requested_app.is_none() {
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
        }
        if requested_app.is_none() {
            requested_app = Some(requested_app::parse(&arg)?);
        } else {
            app_args.push(arg);
        }
    }
    if let Some(requested_app) = requested_app {
        Ok(Args {
            command: Command::RunApp {
                app: requested_app,
                args: app_args,
            },
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

        mod version_parameter {
            use crate::cli::{args, Command};
            use args::Args;

            #[test]
            fn short() {
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

            #[test]
            fn long() {
                let args = vec!["run-that-app", "--version"]
                    .into_iter()
                    .map(ToString::to_string);
                let have = args::parse(args).unwrap();
                let want = Args {
                    log: None,
                    command: Command::DisplayVersion,
                };
                pretty::assert_eq!(have, want);
            }
        }

        mod help_parameter {
            use crate::cli::{args, Command};
            use args::Args;

            #[test]
            fn short() {
                let args = vec!["run-that-app", "-h"]
                    .into_iter()
                    .map(ToString::to_string);
                let have = args::parse(args).unwrap();
                let want = Args {
                    log: None,
                    command: Command::DisplayHelp,
                };
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn long() {
                let args = vec!["run-that-app", "-h"]
                    .into_iter()
                    .map(ToString::to_string);
                let have = args::parse(args).unwrap();
                let want = Args {
                    log: None,
                    command: Command::DisplayHelp,
                };
                pretty::assert_eq!(have, want);
            }
        }

        mod log_parameter {
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
                        args: vec![],
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
                        args: vec![],
                    },
                    log: Some(S("scope")),
                };
                pretty::assert_eq!(have, want);
            }
        }

        mod application_arguments {
            use crate::cli::{args, Command, RequestedApp};
            use args::Args;
            use big_s::S;

            #[test]
            fn no_arguments() {
                let args = vec!["run-that-app", "app@2"]
                    .into_iter()
                    .map(ToString::to_string);
                let have = args::parse(args).unwrap();
                let want = Args {
                    command: Command::RunApp {
                        app: RequestedApp {
                            name: S("app"),
                            version: S("2"),
                        },
                        args: vec![],
                    },
                    log: None,
                };
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn some_arguments() {
                let args = vec!["run-that-app", "app@2", "--switch-1", "--switch-2"]
                    .into_iter()
                    .map(ToString::to_string);
                let have = args::parse(args).unwrap();
                let want = Args {
                    command: Command::RunApp {
                        app: RequestedApp {
                            name: S("app"),
                            version: S("2"),
                        },
                        args: vec![S("--switch-1"), S("--switch-2")],
                    },
                    log: None,
                };
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn rta_and_app_arguments() {
                let args = vec!["run-that-app", "--log", "app@2", "--switch-1", "--switch-2"]
                    .into_iter()
                    .map(ToString::to_string);
                let have = args::parse(args).unwrap();
                let want = Args {
                    command: Command::RunApp {
                        app: RequestedApp {
                            name: S("app"),
                            version: S("2"),
                        },
                        args: vec![S("--switch-1"), S("--switch-2")],
                    },
                    log: Some(S("")),
                };
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn same_arguments_as_run_that_app() {}
        }
    }
}
