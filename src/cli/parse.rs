use super::{AppVersion, Command};
use crate::applications::{ApplicationName, Apps};
use crate::commands::{self, available, run, test, update, versions};
use crate::prelude::*;

#[allow(clippy::too_many_lines)]
pub(crate) fn parse(mut cli_args: impl Iterator<Item = String>, apps: &Apps) -> Result<Command> {
  let _skipped_binary_name = cli_args.next();
  let mut app_version: Option<AppVersion> = None;
  let mut verbose = false;
  let mut app_args: Vec<String> = vec![];
  let mut error_on_output = false;
  let mut include_apps: Vec<ApplicationName> = vec![];
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
        return Ok(Command::AppsLong);
      }
      if &arg == "-a" {
        return Ok(Command::AppsShort);
      }
      if &arg == "--available" {
        indicate_available = true;
        continue;
      }
      if &arg == "--help" || &arg == "-h" {
        return Ok(Command::DisplayHelp);
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
      if &arg == "--verbose" || &arg == "-v" {
        verbose = true;
        continue;
      }
      if &arg == "--version" || &arg == "-V" {
        return Ok(Command::Version);
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
        if key == "--versions" {
          versions = Some(value.parse().map_err(|_| UserError::InvalidNumber)?);
          continue;
        }
        return Err(UserError::UnknownCliOption(arg));
      }
    }
    if app_version.is_none() {
      app_version = Some(AppVersion::new(arg, apps)?);
    } else {
      app_args.push(arg);
    }
  }
  if multiple_true(&[which, indicate_available, setup, test, update, versions.is_some()]) {
    return Err(UserError::MultipleCommandsGiven);
  } else if setup {
    return Ok(Command::Setup);
  } else if update {
    return Ok(Command::Update(update::Args { verbose }));
  }
  if test {
    return Ok(Command::Test(test::Args {
      optional,
      start_at_app: app_version.map(|av| av.app_name),
      verbose,
    }));
  }
  if let Some(AppVersion { app_name, version }) = app_version {
    if indicate_available {
      Ok(Command::Available(available::Args {
        app_name,
        optional,
        version,
        verbose,
      }))
    } else if which {
      Ok(Command::Which(commands::which::Args {
        app_name,
        optional,
        version,
        verbose,
      }))
    } else if let Some(amount) = versions {
      Ok(Command::Versions(versions::Args { app_name, amount, verbose }))
    } else {
      Ok(Command::RunApp(run::Args {
        app_name,
        version,
        app_args,
        error_on_output,
        include_apps,
        optional,
        verbose,
      }))
    }
  } else if error_on_output || optional || verbose || which || indicate_available {
    Err(UserError::MissingApplication)
  } else {
    Ok(Command::DisplayHelp)
  }
}

/// indicates whether the given values contain two or more true values
fn multiple_true(values: &[bool]) -> bool {
  values.iter().filter(|&&value| value).count() >= 2
}

#[cfg(test)]
mod tests {
  use crate::applications::Apps;
  use crate::prelude::*;
  use crate::Command;

  // helper function for tests
  fn parse_args(args: Vec<&'static str>, apps: &Apps) -> Result<Command> {
    super::parse(args.into_iter().map(ToString::to_string), apps)
  }

  mod parse {
    use super::parse_args;
    use crate::applications;
    use crate::cli::Command;

    #[test]
    fn no_arguments() {
      let apps = applications::all();
      let have = parse_args(vec!["rta"], &apps);
      let want = Ok(Command::DisplayHelp);
      pretty::assert_eq!(have, want);
    }

    mod rta_arguments {
      use super::parse_args;
      use crate::applications;
      use crate::cli::Command;
      use crate::commands::run;
      use crate::configuration::Version;
      use crate::prelude::*;
      use big_s::S;

      mod available {
        use super::super::parse_args;
        use crate::applications;
        use crate::cli::Command;
        use crate::commands::available;
        use crate::prelude::*;

        #[test]
        fn with_app() {
          let apps = applications::all();
          let shellcheck = apps.lookup("shellcheck").unwrap();
          let have = parse_args(vec!["rta", "--available", "shellcheck"], &apps);
          let want = Ok(Command::Available(available::Args {
            app_name: shellcheck.app_name(),
            optional: false,
            version: None,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn with_all_options() {
          let apps = applications::all();
          let shellcheck = apps.lookup("shellcheck").unwrap();
          let have = parse_args(vec!["rta", "--available", "--verbose", "shellcheck"], &apps);
          let want = Ok(Command::Available(available::Args {
            app_name: shellcheck.app_name(),
            optional: false,
            version: None,
            verbose: true,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn without_app() {
          let apps = applications::all();
          let have = parse_args(vec!["rta", "--available"], &apps);
          let want = Err(UserError::MissingApplication);
          pretty::assert_eq!(have, want);
        }
      }

      mod error_on_output {
        use super::super::parse_args;
        use crate::applications;
        use crate::cli::Command;
        use crate::commands::run;
        use crate::prelude::*;

        #[test]
        fn normal() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let have = parse_args(vec!["rta", "--error-on-output", "actionlint"], &apps);
          let want = Ok(Command::RunApp(run::Args {
            app_name: actionlint.app_name(),
            version: None,
            app_args: vec![],
            error_on_output: true,
            optional: false,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_app() {
          let apps = applications::all();
          let have = parse_args(vec!["rta", "--error-on-output"], &apps);
          let want = Err(UserError::MissingApplication);
          pretty::assert_eq!(have, want);
        }
      }

      mod test {
        use super::super::parse_args;
        use crate::applications;
        use crate::cli::Command;
        use crate::commands::test;

        #[test]
        fn no_app_no_verbose() {
          let apps = applications::all();
          let have = parse_args(vec!["rta", "--test"], &apps);
          let want = Ok(Command::Test(test::Args {
            optional: false,
            start_at_app: None,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn no_app_verbose() {
          let apps = applications::all();
          let have = parse_args(vec!["rta", "--test", "--verbose"], &apps);
          let want = Ok(Command::Test(test::Args {
            optional: false,
            start_at_app: None,
            verbose: true,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn app_no_verbose() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let have = parse_args(vec!["rta", "--test", "actionlint"], &apps);
          let want = Ok(Command::Test(test::Args {
            optional: false,
            start_at_app: Some(actionlint.app_name()),
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn app_verbose() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let have = parse_args(vec!["rta", "--test", "--verbose", "actionlint"], &apps);
          let want = Ok(Command::Test(test::Args {
            optional: false,
            start_at_app: Some(actionlint.app_name()),
            verbose: true,
          }));
          pretty::assert_eq!(have, want);
        }
      }

      mod help_parameter {
        use super::super::parse_args;
        use crate::applications;
        use crate::cli::Command;

        #[test]
        fn short() {
          let apps = applications::all();
          let have = parse_args(vec!["rta", "-h"], &apps);
          let want = Ok(Command::DisplayHelp);
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn long() {
          let apps = applications::all();
          let have = parse_args(vec!["rta", "--help"], &apps);
          let want = Ok(Command::DisplayHelp);
          pretty::assert_eq!(have, want);
        }
      }

      mod verbose {
        use super::super::parse_args;
        use crate::applications;
        use crate::cli::Command;
        use crate::commands::run;
        use crate::configuration::Version;
        use crate::prelude::*;

        #[test]
        fn long() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let have = parse_args(vec!["rta", "--verbose", "actionlint@2"], &apps);
          let want = Ok(Command::RunApp(run::Args {
            app_name: actionlint.app_name(),
            version: Some(Version::from("2")),
            app_args: vec![],
            error_on_output: false,
            optional: false,
            verbose: true,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn short() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let have = parse_args(vec!["rta", "-v", "actionlint@2"], &apps);
          let want = Ok(Command::RunApp(run::Args {
            app_name: actionlint.app_name(),
            version: Some(Version::from("2")),
            app_args: vec![],
            error_on_output: false,
            optional: false,
            verbose: true,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_app() {
          let apps = applications::all();
          let have = parse_args(vec!["rta", "--verbose"], &apps);
          let want = Err(UserError::MissingApplication);
          pretty::assert_eq!(have, want);
        }
      }

      #[test]
      fn multiple_commands() {
        let apps = applications::all();
        let have = parse_args(vec!["rta", "--which", "--available", "shellcheck"], &apps);
        let want = Err(UserError::MultipleCommandsGiven);
        pretty::assert_eq!(have, want);
      }

      #[test]
      fn optional() {
        let apps = applications::all();
        let actionlint = apps.lookup("actionlint").unwrap();
        let have = parse_args(vec!["rta", "--optional", "actionlint@2", "arg1"], &apps);
        let want = Ok(Command::RunApp(run::Args {
          app_name: actionlint.app_name(),
          version: Some(Version::from("2")),
          app_args: vec![S("arg1")],
          error_on_output: false,
          optional: true,
          verbose: false,
        }));
        pretty::assert_eq!(have, want);
      }

      mod version {
        use super::parse_args;
        use crate::applications;
        use crate::cli::Command;

        #[test]
        fn short() {
          let apps = applications::all();
          let have = parse_args(vec!["rta", "-V"], &apps);
          let want = Ok(Command::Version);
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn long() {
          let apps = applications::all();
          let have = parse_args(vec!["rta", "--version"], &apps);
          let want = Ok(Command::Version);
          pretty::assert_eq!(have, want);
        }
      }

      mod versions {
        use super::parse_args;
        use crate::applications;
        use crate::cli::Command;
        use crate::commands::versions;

        #[test]
        fn correct_usage() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let have = parse_args(vec!["rta", "--versions", "actionlint"], &apps);
          let want = Ok(Command::Versions(versions::Args {
            app_name: actionlint.app_name(),
            amount: 10,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn custom_amount() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let have = parse_args(vec!["rta", "--versions=20", "actionlint"], &apps);
          let want = Ok(Command::Versions(versions::Args {
            app_name: actionlint.app_name(),
            amount: 20,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_app() {
          let apps = applications::all();
          let have = parse_args(vec!["rta", "--versions"], &apps);
          let want = Ok(Command::DisplayHelp);
          pretty::assert_eq!(have, want);
        }
      }

      mod which {
        use super::super::parse_args;
        use crate::cli::Command;
        use crate::prelude::*;
        use crate::{applications, commands};

        #[test]
        fn with_app() {
          let apps = applications::all();
          let shellcheck = apps.lookup("shellcheck").unwrap();
          let have = parse_args(vec!["rta", "--which", "shellcheck"], &apps);
          let want = Ok(Command::Which(commands::which::Args {
            app_name: shellcheck.app_name(),
            optional: false,
            version: None,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn with_all_options() {
          let apps = applications::all();
          let shellcheck = apps.lookup("shellcheck").unwrap();
          let have = parse_args(vec!["rta", "--which", "--verbose", "shellcheck"], &apps);
          let want = Ok(Command::Which(commands::which::Args {
            app_name: shellcheck.app_name(),
            optional: false,
            version: None,
            verbose: true,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn without_app() {
          let apps = applications::all();
          let have = parse_args(vec!["rta", "--which"], &apps);
          let want = Err(UserError::MissingApplication);
          pretty::assert_eq!(have, want);
        }
      }
    }

    mod application_arguments {
      use super::parse_args;
      use crate::applications;
      use crate::cli::Command;
      use crate::commands::run;
      use crate::configuration::Version;
      use big_s::S;

      #[test]
      fn no_arguments() {
        let apps = applications::all();
        let actionlint = apps.lookup("actionlint").unwrap();
        let have = parse_args(vec!["rta", "actionlint@2"], &apps);
        let want = Ok(Command::RunApp(run::Args {
          app_name: actionlint.app_name(),
          version: Some(Version::from("2")),
          app_args: vec![],
          error_on_output: false,
          optional: false,
          verbose: false,
        }));
        pretty::assert_eq!(have, want);
      }

      #[test]
      fn some_arguments() {
        let apps = applications::all();
        let actionlint = apps.lookup("actionlint").unwrap();
        let have = parse_args(vec!["rta", "actionlint@2", "--arg1", "arg2"], &apps);
        let want = Ok(Command::RunApp(run::Args {
          app_name: actionlint.app_name(),
          version: Some(Version::from("2")),
          app_args: vec![S("--arg1"), S("arg2")],
          error_on_output: false,
          optional: false,
          verbose: false,
        }));
        pretty::assert_eq!(have, want);
      }
    }

    mod rta_and_app_arguments {
      use super::parse_args;
      use crate::applications;
      use crate::cli::Command;
      use crate::commands::run;
      use crate::configuration::Version;
      use big_s::S;

      #[test]
      fn rta_and_app_arguments() {
        let apps = applications::all();
        let actionlint = apps.lookup("actionlint").unwrap();
        let have = parse_args(vec!["rta", "--verbose", "actionlint@2", "--arg1", "arg2"], &apps);
        let want = Ok(Command::RunApp(run::Args {
          app_name: actionlint.app_name(),
          version: Some(Version::from("2")),
          app_args: vec![S("--arg1"), S("arg2")],
          error_on_output: false,
          optional: false,
          verbose: true,
        }));
        pretty::assert_eq!(have, want);
      }

      #[test]
      fn same_arguments_as_run_that_app() {
        let apps = applications::all();
        let actionlint = apps.lookup("actionlint").unwrap();
        let have = parse_args(vec!["rta", "actionlint@2", "--verbose", "--version"], &apps);
        let want = Ok(Command::RunApp(run::Args {
          app_name: actionlint.app_name(),
          version: Some(Version::from("2")),
          app_args: vec![S("--verbose"), S("--version")],
          error_on_output: false,
          optional: false,
          verbose: false,
        }));
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
