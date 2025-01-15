use super::{AppVersion, Command};
use crate::commands::{self, available, run, test, update, versions};
use crate::prelude::*;

/// all arguments that can be provided via the CLI
#[derive(Debug, PartialEq)]
pub struct Arguments {
  pub command: Command,
}

#[allow(clippy::too_many_lines)]
pub fn parse(mut cli_args: impl Iterator<Item = String>) -> Result<Arguments> {
  let _skipped_binary_name = cli_args.next();
  let mut app_version: Option<AppVersion> = None;
  let mut verbose = false;
  let mut app_args: Vec<String> = vec![];
  let mut error_on_output = false;
  let mut which = false;
  let mut setup = false;
  let mut test = false;
  let mut indicate_available = false;
  let mut update = false;
  let mut optional = false;
  let mut versions: Option<usize> = None;
  for arg in cli_args {
    if app_version.is_none() {
      if &arg == "--apps" {
        return Ok(Arguments { command: Command::AppsLong });
      }
      if &arg == "-a" {
        return Ok(Arguments { command: Command::AppsShort });
      }
      if &arg == "--available" {
        indicate_available = true;
        continue;
      }
      if &arg == "--help" || &arg == "-h" {
        return Ok(Arguments { command: Command::DisplayHelp });
      }
      if &arg == "--error-on-output" {
        error_on_output = true;
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
      if &arg == "--test" {
        test = true;
        continue;
      }
      if &arg == "--update" {
        update = true;
        continue;
      }
      if &arg == "--version" || &arg == "-V" {
        return Ok(Arguments { command: Command::Version });
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
        if key == "--verbose" || key == "-v" {
          verbose = true;
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
  if multiple_true(&[which, indicate_available, setup, test, update, versions.is_some()]) {
    return Err(UserError::MultipleCommandsGiven);
  } else if setup {
    return Ok(Arguments { command: Command::Setup });
  } else if update {
    return Ok(Arguments {
      command: Command::Update(update::Args { verbose }),
    });
  }
  if test {
    return Ok(Arguments {
      command: Command::Test(test::Args {
        optional,
        start_at_app: app_version.map(|av| av.app_name),
        verbose,
      }),
    });
  }
  if let Some(AppVersion { app_name, version }) = app_version {
    if indicate_available {
      Ok(Arguments {
        command: Command::Available(available::Args {
          app_name,
          optional,
          version,
          verbose,
        }),
      })
    } else if which {
      Ok(Arguments {
        command: Command::Which(commands::which::Args {
          app_name,
          optional,
          version,
          verbose,
        }),
      })
    } else if let Some(amount) = versions {
      Ok(Arguments {
        command: Command::Versions(versions::Args { app_name, amount, verbose }),
      })
    } else {
      Ok(Arguments {
        command: Command::RunApp(run::Args {
          app_name,
          version,
          app_args,
          error_on_output,
          optional,
          verbose,
        }),
      })
    }
  } else if error_on_output || optional || verbose || which || indicate_available {
    Err(UserError::MissingApplication)
  } else {
    Ok(Arguments { command: Command::DisplayHelp })
  }
}

/// indicates whether the given values contain two or more true values
fn multiple_true(values: &[bool]) -> bool {
  values.iter().filter(|&&value| value).count() >= 2
}

#[cfg(test)]
mod tests {
  use super::Arguments;
  use crate::prelude::*;

  // helper function for tests
  fn parse_args(args: Vec<&'static str>) -> Result<Arguments> {
    super::parse(args.into_iter().map(ToString::to_string))
  }

  mod parse {
    use super::parse_args;
    use crate::cli::{Arguments, Command};

    #[test]
    fn no_arguments() {
      let have = parse_args(vec!["rta"]);
      let want = Ok(Arguments { command: Command::DisplayHelp });
      pretty::assert_eq!(have, want);
    }

    mod rta_arguments {
      use super::parse_args;
      use crate::cli::{Arguments, Command};
      use crate::commands::run;
      use crate::configuration::{ApplicationName, Version};
      use crate::prelude::*;
      use big_s::S;

      mod available {
        use super::super::parse_args;
        use crate::cli::{Arguments, Command};
        use crate::commands::available;
        use crate::configuration::ApplicationName;
        use crate::prelude::*;

        #[test]
        fn with_app() {
          let have = parse_args(vec!["rta", "--available", "shellcheck"]);
          let want = Ok(Arguments {
            command: Command::Available(available::Args {
              app_name: ApplicationName::from("shellcheck"),
              optional: false,
              version: None,
              verbose: false,
            }),
          });
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn with_all_options() {
          let have = parse_args(vec!["rta", "--available", "--verbose", "shellcheck"]);
          let want = Ok(Arguments {
            command: Command::Available(available::Args {
              app_name: ApplicationName::from("shellcheck"),
              optional: false,
              version: None,
              verbose: true,
            }),
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
        use crate::cli::{Arguments, Command};
        use crate::commands::run;
        use crate::configuration::ApplicationName;
        use crate::prelude::*;

        #[test]
        fn normal() {
          let have = parse_args(vec!["rta", "--error-on-output", "app"]);
          let want = Ok(Arguments {
            command: Command::RunApp(run::Args {
              app_name: ApplicationName::from("app"),
              version: None,
              app_args: vec![],
              error_on_output: true,
              optional: false,
              verbose: false,
            }),
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

      mod test {
        use super::super::parse_args;
        use crate::cli::{Arguments, Command};
        use crate::commands::test;
        use crate::configuration::ApplicationName;

        #[test]
        fn no_app_no_verbose() {
          let have = parse_args(vec!["rta", "--test"]);
          let want = Ok(Arguments {
            command: Command::Test(test::Args {
              optional: false,
              start_at_app: None,
              verbose: false,
            }),
          });
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn no_app_verbose() {
          let have = parse_args(vec!["rta", "--test", "--verbose"]);
          let want = Ok(Arguments {
            command: Command::Test(test::Args {
              optional: false,
              start_at_app: None,
              verbose: true,
            }),
          });
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn app_no_verbose() {
          let have = parse_args(vec!["rta", "--test", "actionlint"]);
          let want = Ok(Arguments {
            command: Command::Test(test::Args {
              optional: false,
              start_at_app: Some(ApplicationName::from("actionlint")),
              verbose: false,
            }),
          });
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn app_verbose() {
          let have = parse_args(vec!["rta", "--test", "--verbose", "actionlint"]);
          let want = Ok(Arguments {
            command: Command::Test(test::Args {
              optional: false,
              start_at_app: Some(ApplicationName::from("actionlint")),
              verbose: true,
            }),
          });
          pretty::assert_eq!(have, want);
        }
      }

      mod help_parameter {
        use super::super::parse_args;
        use crate::cli::{Arguments, Command};

        #[test]
        fn short() {
          let have = parse_args(vec!["rta", "-h"]);
          let want = Ok(Arguments { command: Command::DisplayHelp });
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn long() {
          let have = parse_args(vec!["rta", "--help"]);
          let want = Ok(Arguments { command: Command::DisplayHelp });
          pretty::assert_eq!(have, want);
        }
      }

      mod verbose {
        use super::super::parse_args;
        use crate::cli::{Arguments, Command};
        use crate::commands::run;
        use crate::configuration::{ApplicationName, Version};
        use crate::prelude::*;

        #[test]
        fn long() {
          let have = parse_args(vec!["rta", "--verbose", "app@2"]);
          let want = Ok(Arguments {
            command: Command::RunApp(run::Args {
              app_name: ApplicationName::from("app"),
              version: Some(Version::from("2")),
              app_args: vec![],
              error_on_output: false,
              optional: false,
              verbose: true,
            }),
          });
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn short() {
          let have = parse_args(vec!["rta", "-v", "app@2"]);
          let want = Ok(Arguments {
            command: Command::RunApp(run::Args {
              app_name: ApplicationName::from("app"),
              version: Some(Version::from("2")),
              app_args: vec![],
              error_on_output: false,
              optional: false,
              verbose: true,
            }),
          });
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_app() {
          let have = parse_args(vec!["rta", "--verbose"]);
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
        let want = Ok(Arguments {
          command: Command::RunApp(run::Args {
            app_name: ApplicationName::from("app"),
            version: Some(Version::from("2")),
            app_args: vec![S("arg1")],
            error_on_output: false,
            optional: true,
            verbose: false,
          }),
        });
        pretty::assert_eq!(have, want);
      }

      mod version {
        use super::parse_args;
        use crate::cli::{arguments, Command};
        use arguments::Arguments;

        #[test]
        fn short() {
          let have = parse_args(vec!["rta", "-V"]);
          let want = Ok(Arguments { command: Command::Version });
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn long() {
          let have = parse_args(vec!["rta", "--version"]);
          let want = Ok(Arguments { command: Command::Version });
          pretty::assert_eq!(have, want);
        }
      }

      mod versions {
        use super::parse_args;
        use crate::cli::{arguments, Command};
        use crate::commands::versions;
        use crate::configuration::ApplicationName;
        use arguments::Arguments;

        #[test]
        fn correct_usage() {
          let have = parse_args(vec!["rta", "--versions", "actionlint"]);
          let want = Ok(Arguments {
            command: Command::Versions(versions::Args {
              app_name: ApplicationName::from("actionlint"),
              amount: 10,
              verbose: false,
            }),
          });
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn custom_amount() {
          let have = parse_args(vec!["rta", "--versions=20", "actionlint"]);
          let want = Ok(Arguments {
            command: Command::Versions(versions::Args {
              app_name: ApplicationName::from("actionlint"),
              amount: 20,
              verbose: false,
            }),
          });
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_app() {
          let have = parse_args(vec!["rta", "--versions"]);
          let want = Ok(Arguments { command: Command::DisplayHelp });
          pretty::assert_eq!(have, want);
        }
      }

      mod which {
        use super::super::parse_args;
        use crate::cli::{Arguments, Command};
        use crate::commands;
        use crate::configuration::ApplicationName;
        use crate::prelude::*;

        #[test]
        fn with_app() {
          let have = parse_args(vec!["rta", "--which", "shellcheck"]);
          let want = Ok(Arguments {
            command: Command::Which(commands::which::Args {
              app_name: ApplicationName::from("shellcheck"),
              optional: false,
              version: None,
              verbose: false,
            }),
          });
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn with_all_options() {
          let have = parse_args(vec!["rta", "--which", "--verbose", "shellcheck"]);
          let want = Ok(Arguments {
            command: Command::Which(commands::which::Args {
              app_name: ApplicationName::from("shellcheck"),
              optional: false,
              version: None,
              verbose: true,
            }),
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
      use crate::cli::{arguments, Command};
      use crate::commands::run;
      use crate::configuration::{ApplicationName, Version};
      use arguments::Arguments;
      use big_s::S;

      #[test]
      fn no_arguments() {
        let have = parse_args(vec!["rta", "app@2"]);
        let want = Ok(Arguments {
          command: Command::RunApp(run::Args {
            app_name: ApplicationName::from("app"),
            version: Some(Version::from("2")),
            app_args: vec![],
            error_on_output: false,
            optional: false,
            verbose: false,
          }),
        });
        pretty::assert_eq!(have, want);
      }

      #[test]
      fn some_arguments() {
        let have = parse_args(vec!["rta", "app@2", "--arg1", "arg2"]);
        let want = Ok(Arguments {
          command: Command::RunApp(run::Args {
            app_name: ApplicationName::from("app"),
            version: Some(Version::from("2")),
            app_args: vec![S("--arg1"), S("arg2")],
            error_on_output: false,
            optional: false,
            verbose: false,
          }),
        });
        pretty::assert_eq!(have, want);
      }
    }

    mod rta_and_app_arguments {
      use super::parse_args;
      use crate::cli::{Arguments, Command};
      use crate::commands::run;
      use crate::configuration::{ApplicationName, Version};
      use big_s::S;

      #[test]
      fn rta_and_app_arguments() {
        let have = parse_args(vec!["rta", "--verbose", "app@2", "--arg1", "arg2"]);
        let want = Ok(Arguments {
          command: Command::RunApp(run::Args {
            app_name: ApplicationName::from("app"),
            version: Some(Version::from("2")),
            app_args: vec![S("--arg1"), S("arg2")],
            error_on_output: false,
            optional: false,
            verbose: true,
          }),
        });
        pretty::assert_eq!(have, want);
      }

      #[test]
      fn same_arguments_as_run_that_app() {
        let have = parse_args(vec!["rta", "app@2", "--verbose", "--version"]);
        let want = Ok(Arguments {
          command: Command::RunApp(run::Args {
            app_name: ApplicationName::from("app"),
            version: Some(Version::from("2")),
            app_args: vec![S("--verbose"), S("--version")],
            error_on_output: false,
            optional: false,
            verbose: false,
          }),
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
