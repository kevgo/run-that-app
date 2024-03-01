use super::Command;
use crate::cmd::run;
use crate::config::AppVersion;
use crate::{Result, UserError};

/// all arguments that can be provided via the CLI
#[derive(Debug, PartialEq)]
pub struct Args {
    pub command: Command,
}

#[allow(clippy::too_many_lines)]
pub fn parse(mut cli_args: impl Iterator<Item = String>) -> Result<Args> {
    let _skipped_binary_name = cli_args.next();
    let mut app_version: Option<AppVersion> = None;
    let mut log: Option<String> = None;
    let mut app_args: Vec<String> = vec![];
    let mut error_on_output = false;
    let mut include_path = false;
    let mut which = false;
    let mut setup = false;
    let mut indicate_available = false;
    let mut update = false;
    let mut optional = false;
    let mut versions: Option<usize> = None;
    for arg in cli_args {
        if app_version.is_none() {
            if &arg == "--available" {
                indicate_available = true;
                continue;
            }
            if &arg == "--help" || &arg == "-h" {
                return Ok(Args { command: Command::DisplayHelp });
            }
            if &arg == "--error-on-output" {
                error_on_output = true;
                continue;
            }
            if &arg == "--include-path" {
                include_path = true;
                continue;
            }
            if &arg == "--optional" {
                optional = true;
                continue;
            }
            if &arg == "--setup" {
                setup = true;
                continue;
            }
            if &arg == "--update" {
                update = true;
                continue;
            }
            if &arg == "--version" || &arg == "-V" {
                return Ok(Args { command: Command::Version });
            }
            if &arg == "--versions" {
                versions = Some(10);
                continue;
            }
            if &arg == "--which" {
                which = true;
                continue;
            }
            if arg.starts_with('-') {
                let (key, value) = arg.split_once('=').unwrap_or((&arg, ""));
                if key == "--log" || key == "-l" {
                    log = Some(value.to_string());
                    continue;
                }
                if key == "--versions" {
                    versions = Some(value.parse().map_err(|_| UserError::InvalidNumber)?);
                    continue;
                }
                return Err(UserError::UnknownCliOption(arg));
            }
        }
        if app_version.is_none() {
            app_version = Some(AppVersion::new(arg));
        } else {
            app_args.push(arg);
        }
    }
    if multiple_true(&[which, indicate_available, setup, update, versions.is_some()]) {
        return Err(UserError::MultipleCommandsGiven);
    } else if setup {
        return Ok(Args { command: Command::Setup });
    } else if update {
        return Ok(Args {
            command: Command::Update { log },
        });
    }
    if let Some(app) = app_version {
        if indicate_available {
            Ok(Args {
                command: Command::Available { app, include_path, log },
            })
        } else if which {
            Ok(Args {
                command: Command::Which { app, include_path, log },
            })
        } else if let Some(amount) = versions {
            Ok(Args {
                command: Command::Versions { app: app.name, amount, log },
            })
        } else {
            Ok(Args {
                command: Command::RunApp {
                    data: run::Data {
                        app_version: app,
                        app_args,
                        error_on_output,
                        include_path,
                        optional,
                    },
                    log,
                },
            })
        }
    } else if error_on_output || include_path || optional || log.is_some() || which || indicate_available {
        Err(UserError::MissingApplication)
    } else {
        Ok(Args { command: Command::DisplayHelp })
    }
}

/// indicates whether the given values contain two or more true values
fn multiple_true(values: &[bool]) -> bool {
    values.iter().filter(|&&value| value).count() >= 2
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
        use crate::cli::{Args, Command};

        #[test]
        fn no_arguments() {
            let have = parse_args(vec!["rta"]);
            let want = Ok(Args { command: Command::DisplayHelp });
            pretty::assert_eq!(have, want);
        }

        mod rta_arguments {
            use big_s::S;

            use super::parse_args;
            use crate::cli::{Args, Command};
            use crate::cmd::run;
            use crate::config::{AppName, AppVersion, Version};
            use crate::error::UserError;

            mod available {
                use super::super::parse_args;
                use crate::cli::{Args, Command};
                use crate::config::{AppName, AppVersion, Version};
                use crate::error::UserError;
                use big_s::S;

                #[test]
                fn with_app() {
                    let have = parse_args(vec!["rta", "--available", "shellcheck"]);
                    let want = Ok(Args {
                        command: Command::Available {
                            app: AppVersion {
                                name: AppName::from("shellcheck"),
                                version: Version::None,
                            },
                            include_path: false,
                            log: None,
                        },
                    });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn with_all_options() {
                    let have = parse_args(vec!["rta", "--available", "--include-path", "--log=detect", "shellcheck"]);
                    let want = Ok(Args {
                        command: Command::Available {
                            app: AppVersion {
                                name: AppName::from("shellcheck"),
                                version: Version::None,
                            },
                            include_path: true,
                            log: Some(S("detect")),
                        },
                    });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn without_app() {
                    let have = parse_args(vec!["rta", "--available"]);
                    let want = Err(UserError::MissingApplication);
                    pretty::assert_eq!(have, want);
                }
            }

            mod error_on_output {
                use super::super::parse_args;
                use crate::cli::{Args, Command};
                use crate::cmd::run;
                use crate::config::{AppName, AppVersion, Version};
                use crate::error::UserError;

                #[test]
                fn normal() {
                    let have = parse_args(vec!["rta", "--error-on-output", "app"]);
                    let want = Ok(Args {
                        command: Command::RunApp {
                            data: run::Data {
                                app_version: AppVersion {
                                    name: AppName::from("app"),
                                    version: Version::None,
                                },
                                app_args: vec![],
                                error_on_output: true,
                                include_path: false,
                                optional: false,
                            },
                            log: None,
                        },
                    });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn missing_app() {
                    let have = parse_args(vec!["rta", "--error-on-output"]);
                    let want = Err(UserError::MissingApplication);
                    pretty::assert_eq!(have, want);
                }
            }

            mod help_parameter {
                use super::super::parse_args;
                use crate::cli::{Args, Command};

                #[test]
                fn short() {
                    let have = parse_args(vec!["rta", "-h"]);
                    let want = Ok(Args { command: Command::DisplayHelp });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn long() {
                    let have = parse_args(vec!["rta", "--help"]);
                    let want = Ok(Args { command: Command::DisplayHelp });
                    pretty::assert_eq!(have, want);
                }
            }

            mod include_path {
                use super::super::parse_args;
                use crate::cli::{Args, Command};
                use crate::cmd::run;
                use crate::config::{AppName, AppVersion, Version};
                use crate::UserError;
                use big_s::S;

                #[test]
                fn with_app() {
                    let have = parse_args(vec!["rta", "--include-path", "app@2", "arg1"]);
                    let want = Ok(Args {
                        command: Command::RunApp {
                            data: run::Data {
                                app_version: AppVersion {
                                    name: AppName::from("app"),
                                    version: Version::from("2"),
                                },
                                app_args: vec![S("arg1")],
                                error_on_output: false,
                                include_path: true,
                                optional: false,
                            },
                            log: None,
                        },
                    });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn without_app() {
                    let have = parse_args(vec!["rta", "--include-path"]);
                    let want = Err(UserError::MissingApplication);
                    pretty::assert_eq!(have, want);
                }
            }

            mod log {
                use super::super::parse_args;
                use crate::cli::{Args, Command};
                use crate::cmd::run;
                use crate::config::{AppName, AppVersion, Version};
                use crate::error::UserError;
                use big_s::S;

                #[test]
                fn everything() {
                    let have = parse_args(vec!["rta", "--log", "app@2"]);
                    let want = Ok(Args {
                        command: Command::RunApp {
                            data: run::Data {
                                app_version: AppVersion {
                                    name: AppName::from("app"),
                                    version: Version::from("2"),
                                },
                                app_args: vec![],
                                error_on_output: false,
                                include_path: false,
                                optional: false,
                            },
                            log: Some(S("")),
                        },
                    });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn limited() {
                    let have = parse_args(vec!["rta", "--log=scope", "app@2"]);
                    let want = Ok(Args {
                        command: Command::RunApp {
                            data: run::Data {
                                app_version: AppVersion {
                                    name: AppName::from("app"),
                                    version: Version::from("2"),
                                },
                                app_args: vec![],
                                error_on_output: false,
                                include_path: false,
                                optional: false,
                            },
                            log: Some(S("scope")),
                        },
                    });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn missing_app() {
                    let have = parse_args(vec!["rta", "--log"]);
                    let want = Err(UserError::MissingApplication);
                    pretty::assert_eq!(have, want);
                }
            }

            #[test]
            fn multiple_commands() {
                let have = parse_args(vec!["rta", "--which", "--available", "shellcheck"]);
                let want = Err(UserError::MultipleCommandsGiven);
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn optional() {
                let have = parse_args(vec!["rta", "--optional", "app@2", "arg1"]);
                let want = Ok(Args {
                    command: Command::RunApp {
                        data: run::Data {
                            app_version: AppVersion {
                                name: AppName::from("app"),
                                version: Version::from("2"),
                            },
                            app_args: vec![S("arg1")],
                            error_on_output: false,
                            include_path: false,
                            optional: true,
                        },
                        log: None,
                    },
                });
                pretty::assert_eq!(have, want);
            }

            mod version {
                use super::parse_args;
                use crate::cli::{args, Command};
                use args::Args;

                #[test]
                fn short() {
                    let have = parse_args(vec!["rta", "-V"]);
                    let want = Ok(Args { command: Command::Version });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn long() {
                    let have = parse_args(vec!["rta", "--version"]);
                    let want = Ok(Args { command: Command::Version });
                    pretty::assert_eq!(have, want);
                }
            }

            mod versions {
                use super::parse_args;
                use crate::cli::{args, Command};
                use crate::config::AppName;
                use args::Args;

                #[test]
                fn correct_usage() {
                    let have = parse_args(vec!["rta", "--versions", "actionlint"]);
                    let want = Ok(Args {
                        command: Command::Versions {
                            app: AppName::from("actionlint"),
                            amount: 10,
                            log: None,
                        },
                    });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn custom_amount() {
                    let have = parse_args(vec!["rta", "--versions=20", "actionlint"]);
                    let want = Ok(Args {
                        command: Command::Versions {
                            app: AppName::from("actionlint"),
                            amount: 20,
                            log: None,
                        },
                    });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn missing_app() {
                    let have = parse_args(vec!["rta", "--versions"]);
                    let want = Ok(Args { command: Command::DisplayHelp });
                    pretty::assert_eq!(have, want);
                }
            }

            mod which {
                use super::super::parse_args;
                use crate::cli::{Args, Command};
                use crate::config::{AppName, AppVersion, Version};
                use crate::UserError;
                use big_s::S;

                #[test]
                fn with_app() {
                    let have = parse_args(vec!["rta", "--which", "shellcheck"]);
                    let want = Ok(Args {
                        command: Command::Which {
                            app: AppVersion {
                                name: AppName::from("shellcheck"),
                                version: Version::None,
                            },
                            include_path: false,
                            log: None,
                        },
                    });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn with_all_options() {
                    let have = parse_args(vec!["rta", "--which", "--include-path", "--log=detect", "shellcheck"]);
                    let want = Ok(Args {
                        command: Command::Which {
                            app: AppVersion {
                                name: AppName::from("shellcheck"),
                                version: Version::None,
                            },
                            include_path: true,
                            log: Some(S("detect")),
                        },
                    });
                    pretty::assert_eq!(have, want);
                }

                #[test]
                fn without_app() {
                    let have = parse_args(vec!["rta", "--which"]);
                    let want = Err(UserError::MissingApplication);
                    pretty::assert_eq!(have, want);
                }
            }
        }

        mod application_arguments {
            use super::parse_args;
            use crate::cli::{args, Command};
            use crate::cmd::run;
            use crate::config::{AppName, AppVersion, Version};
            use args::Args;
            use big_s::S;

            #[test]
            fn no_arguments() {
                let have = parse_args(vec!["rta", "app@2"]);
                let want = Ok(Args {
                    command: Command::RunApp {
                        data: run::Data {
                            app_version: AppVersion {
                                name: AppName::from("app"),
                                version: Version::from("2"),
                            },
                            app_args: vec![],
                            error_on_output: false,
                            include_path: false,
                            optional: false,
                        },
                        log: None,
                    },
                });
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn some_arguments() {
                let have = parse_args(vec!["rta", "app@2", "--arg1", "arg2"]);
                let want = Ok(Args {
                    command: Command::RunApp {
                        data: run::Data {
                            app_version: AppVersion {
                                name: AppName::from("app"),
                                version: Version::from("2"),
                            },
                            app_args: vec![S("--arg1"), S("arg2")],
                            error_on_output: false,
                            include_path: false,
                            optional: false,
                        },
                        log: None,
                    },
                });
                pretty::assert_eq!(have, want);
            }
        }

        mod rta_and_app_arguments {
            use super::parse_args;
            use crate::cli::{Args, Command};
            use crate::cmd::run;
            use crate::config::{AppName, AppVersion, Version};
            use big_s::S;

            #[test]
            fn rta_and_app_arguments() {
                let have = parse_args(vec!["rta", "--log=l1", "app@2", "--arg1", "arg2"]);
                let app = AppVersion {
                    name: AppName::from("app"),
                    version: Version::from("2"),
                };
                let want = Ok(Args {
                    command: Command::RunApp {
                        data: run::Data {
                            app_version: app,
                            app_args: vec![S("--arg1"), S("arg2")],
                            error_on_output: false,
                            include_path: false,
                            optional: false,
                        },
                        log: Some(S("l1")),
                    },
                });
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn same_arguments_as_run_that_app() {
                let have = parse_args(vec!["rta", "app@2", "--log=app", "--version"]);
                let want = Ok(Args {
                    command: Command::RunApp {
                        data: run::Data {
                            app_version: AppVersion {
                                name: AppName::from("app"),
                                version: Version::from("2"),
                            },
                            app_args: vec![S("--log=app"), S("--version")],
                            error_on_output: false,
                            include_path: false,
                            optional: false,
                        },
                        log: None,
                    },
                });
                pretty::assert_eq!(have, want);
            }
        }
    }

    mod multiple_true {
        use super::super::multiple_true;

        #[test]
        fn none_true() {
            assert!(!multiple_true(&[false, false, false]));
        }

        #[test]
        fn one_true() {
            assert!(!multiple_true(&[false, true, false]));
        }

        #[test]
        fn two_true() {
            assert!(multiple_true(&[true, true, false]));
        }

        #[test]
        fn all_true() {
            assert!(multiple_true(&[true, true, true]));
        }
    }
}
