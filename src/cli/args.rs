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
            run_request = Some(parse_runrequest(&arg));
        } else {
            return Err(UserError::DuplicateRunRequest);
        }
    }
    if let Some(request) = run_request {
        Ok(Args {
            command: Command::RunApp { request },
            log,
        })
    } else {
        Ok(Args {
            command: Command::DisplayHelp,
            log,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Args {
    pub command: Command,
    pub log: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    RunApp { request: RunRequest },
    DisplayHelp,
    DisplayVersion,
}

/// a request from the user to run a particular app
#[derive(Debug, PartialEq)]
pub struct RunRequest {
    pub name: String,
    pub version: String,
}

pub fn parse_runrequest(token: &str) -> RunRequest {
    let (app_name, version) = token.split_once('@').unwrap_or((token, ""));
    RunRequest {
        name: app_name.to_string(),
        version: version.to_string(),
    }
}

#[cfg(test)]
mod tests {

    mod parse {
        use crate::cli::{parse, Args, Command};

        #[test]
        fn no_arguments() {
            let args = vec!["run-that-app"].into_iter().map(ToString::to_string);
            let have = parse(args).unwrap();
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
            let have = parse(args).unwrap();
            let want = Args {
                log: None,
                command: Command::DisplayVersion,
            };
            pretty::assert_eq!(have, want);
        }

        mod logging {
            use crate::cli::{parse, Args, Command, RunRequest};
            use big_s::S;

            #[test]
            fn everything() {
                let args = vec!["run-that-app", "--log", "app"]
                    .into_iter()
                    .map(ToString::to_string);
                let have = parse(args).unwrap();
                let want = Args {
                    command: Command::RunApp {
                        request: RunRequest {
                            name: S("app"),
                            version: S(""),
                        },
                    },
                    log: Some(S("")),
                };
                pretty::assert_eq!(have, want);
            }

            #[test]
            fn limited() {
                let args = vec!["run-that-app", "--log=scope", "app"]
                    .into_iter()
                    .map(ToString::to_string);
                let have = parse(args).unwrap();
                let want = Args {
                    command: Command::RunApp {
                        request: RunRequest {
                            name: S("app"),
                            version: S(""),
                        },
                    },
                    log: Some(S("scope")),
                };
                pretty::assert_eq!(have, want);
            }
        }
    }

    mod parse_runrequest {
        use crate::cli::args::parse_runrequest;
        use crate::cli::RunRequest;
        use big_s::S;

        #[test]
        fn name_and_version() {
            let give = "shellcheck@0.9.0";
            let have = parse_runrequest(give);
            let want = RunRequest {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_only() {
            let give = "shellcheck";
            let have = parse_runrequest(give);
            let want = RunRequest {
                name: S("shellcheck"),
                version: S(""),
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn empty_version() {
            let give = "shellcheck@";
            let have = parse_runrequest(give);
            let want = RunRequest {
                name: S("shellcheck"),
                version: S(""),
            };
            pretty::assert_eq!(have, want);
        }
    }
}
