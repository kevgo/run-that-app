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
    let mut include_global = false;
    let mut optional = false;
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
            if &arg == "--include-global" {
                include_global = true;
                continue;
            }
            if &arg == "--optional" {
                optional = true;
                continue;
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
            requested_app = Some(requested_app::parse(&arg));
        } else {
            app_args.push(arg);
        }
    }
    let command = match requested_app {
        Some(app) => Command::RunApp {
            app,
            args: app_args,
            include_global,
            optional,
        },
        None => Command::DisplayHelp,
    };
    Ok(Args { command, log })
}

#[cfg(test)]
mod tests {
    use super::Args;
    use crate::Result;

    // helper function for tests
    fn parse_args(args: Vec<&'static str>) -> Result<Args> {
        super::parse(args.into_iter().map(ToString::to_string))
    }

    mod parse {
        use super::parse_args;
        use crate::cli::{Args, Command, RequestedApp};
        use big_s::S;

        #[test]
        fn no_arguments() {
            let have = parse_args(vec!["run-that-app"]);
            let want = Ok(Args {
                log: None,
                command: Command::DisplayHelp,
            });
            pretty::assert_eq!(have, want);
        }

        mod version_parameter {
            use super::parse_args;
            use crate::cli::{args, Command};
            use args::Args;

            #[test]
            fn short() {
                let have = parse_args(vec!["run-that-app", "-V"]);
                let want = Ok(Args {
                    log: None,
                    command: Command::DisplayVersion,
                });
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn long() {
                let have = parse_args(vec!["run-that-app", "--version"]);
                let want = Ok(Args {
                    log: None,
                    command: Command::DisplayVersion,
                });
                pretty::assert_eq!(have, want);
            }
        }

        mod help_parameter {
            use super::parse_args;
            use crate::cli::{args, Command};
            use args::Args;

            #[test]
            fn short() {
                let have = parse_args(vec!["run-that-app", "-h"]);
                let want = Ok(Args {
                    log: None,
                    command: Command::DisplayHelp,
                });
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn long() {
                let have = parse_args(vec!["run-that-app", "-h"]);
                let want = Ok(Args {
                    log: None,
                    command: Command::DisplayHelp,
                });
                pretty::assert_eq!(have, want);
            }
        }

        #[test]
        fn include_global() {
            let have = parse_args(vec!["run-that-app", "--include-global", "app@2", "arg1"]);
            let want = Ok(Args {
                command: Command::RunApp {
                    app: RequestedApp {
                        name: S("app"),
                        version: S("2"),
                    },
                    args: vec![S("arg1")],
                    include_global: true,
                    optional: false,
                },
                log: None,
            });
            pretty::assert_eq!(have, want);
        }

        mod log_parameter {
            use super::parse_args;
            use crate::cli::{Args, Command, RequestedApp};
            use big_s::S;

            #[test]
            fn everything() {
                let have = parse_args(vec!["run-that-app", "--log", "app@2"]);
                let want = Ok(Args {
                    command: Command::RunApp {
                        app: RequestedApp {
                            name: S("app"),
                            version: S("2"),
                        },
                        args: vec![],
                        include_global: false,
                        optional: false,
                    },
                    log: Some(S("")),
                });
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn limited() {
                let have = parse_args(vec!["run-that-app", "--log=scope", "app@2"]);
                let want = Ok(Args {
                    command: Command::RunApp {
                        app: RequestedApp {
                            name: S("app"),
                            version: S("2"),
                        },
                        args: vec![],
                        include_global: false,
                        optional: false,
                    },
                    log: Some(S("scope")),
                });
                pretty::assert_eq!(have, want);
            }
        }

        #[test]
        fn optional() {
            let have = parse_args(vec!["run-that-app", "--optional", "app@2", "arg1"]);
            let want = Ok(Args {
                command: Command::RunApp {
                    app: RequestedApp {
                        name: S("app"),
                        version: S("2"),
                    },
                    args: vec![S("arg1")],
                    include_global: false,
                    optional: true,
                },
                log: None,
            });
            pretty::assert_eq!(have, want);
        }

        mod application_arguments {
            use super::parse_args;
            use crate::cli::{args, Command, RequestedApp};
            use args::Args;
            use big_s::S;

            #[test]
            fn no_arguments() {
                let have = parse_args(vec!["run-that-app", "app@2"]);
                let want = Ok(Args {
                    command: Command::RunApp {
                        app: RequestedApp {
                            name: S("app"),
                            version: S("2"),
                        },
                        args: vec![],
                        include_global: false,
                        optional: false,
                    },
                    log: None,
                });
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn some_arguments() {
                let have = parse_args(vec!["run-that-app", "app@2", "--arg1", "arg2"]);
                let want = Ok(Args {
                    command: Command::RunApp {
                        app: RequestedApp {
                            name: S("app"),
                            version: S("2"),
                        },
                        args: vec![S("--arg1"), S("arg2")],
                        include_global: false,
                        optional: false,
                    },
                    log: None,
                });
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn rta_and_app_arguments() {
                let have = parse_args(vec!["run-that-app", "--log=l1", "app@2", "--arg1", "arg2"]);
                let want = Ok(Args {
                    command: Command::RunApp {
                        app: RequestedApp {
                            name: S("app"),
                            version: S("2"),
                        },
                        args: vec![S("--arg1"), S("arg2")],
                        include_global: false,
                        optional: false,
                    },
                    log: Some(S("l1")),
                });
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn same_arguments_as_run_that_app() {
                let have = parse_args(vec!["run-that-app", "app@2", "--log=app", "--version"]);
                let want = Ok(Args {
                    command: Command::RunApp {
                        app: RequestedApp {
                            name: S("app"),
                            version: S("2"),
                        },
                        args: vec![S("--log=app"), S("--version")],
                        include_global: false,
                        optional: false,
                    },
                    log: None,
                });
                pretty::assert_eq!(have, want);
            }
        }
    }
}
