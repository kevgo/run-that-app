use super::{AppVersion, Command};
use crate::applications::{ApplicationName, Apps};
use crate::commands::{self, add, available, run, test, update, versions};
use crate::error::{Result, UserError};

#[allow(clippy::too_many_lines)]
pub(crate) fn parse(cli_args: impl Iterator<Item = String>, apps: &Apps) -> Result<Command> {
  let mut app_version: Option<AppVersion> = None;
  let mut verbose = false;
  let mut app_args: Vec<String> = vec![];
  let mut error_on_output = false;
  let mut from_source = false;
  let mut include_apps: Vec<ApplicationName> = vec![];
  let mut which = false;
  let mut add = false;
  let mut install = false;
  let mut test = false;
  let mut indicate_available = false;
  let mut update = false;
  let mut optional = false;
  let mut versions: Option<usize> = None;
  for arg in cli_args {
    if app_version.is_none() {
      if &arg == "--add" {
        add = true;
        continue;
      }
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
      if &arg == "--from-source" {
        from_source = true;
        continue;
      }
      if &arg == "--help" || &arg == "-h" {
        return Ok(Command::DisplayHelp);
      }
      if &arg == "--error-on-output" {
        error_on_output = true;
        continue;
      }
      if &arg == "--install" {
        install = true;
        continue;
      }
      if &arg == "--optional" {
        optional = true;
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
        if key == "--include" {
          let app = apps.lookup(value)?;
          include_apps.push(app.name());
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
      app_version = Some(AppVersion::new(arg, apps)?);
    } else {
      app_args.push(arg);
    }
  }
  if multiple_true(&[which, indicate_available, install, test, update, versions.is_some()]) {
    return Err(UserError::MultipleCommandsGiven);
  }
  if update {
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
    if add {
      return Ok(Command::Add(add::Args { app_name, verbose }));
    }
    if indicate_available {
      return Ok(Command::Available(available::Args {
        app_name,
        optional,
        version,
        verbose,
      }));
    }
    if install {
      return Ok(Command::Install(commands::install::Args {
        app_name,
        version,
        from_source,
        include_apps,
        optional,
        verbose,
      }));
    }
    if which {
      return Ok(Command::Which(commands::which::Args {
        app_name,
        optional,
        version,
        verbose,
      }));
    }
    if let Some(amount) = versions {
      return Ok(Command::Versions(versions::Args { app_name, amount, verbose }));
    }
    return Ok(Command::RunApp(run::Args {
      app_name,
      version,
      app_args,
      error_on_output,
      from_source,
      include_apps,
      optional,
      verbose,
    }));
  }
  if error_on_output || install || optional || verbose || which || indicate_available {
    return Err(UserError::MissingApplication);
  }
  Ok(Command::DisplayHelp)
}

/// indicates whether the given values contain two or more true values
fn multiple_true(values: &[bool]) -> bool {
  values.iter().filter(|&&value| value).count() >= 2
}

#[cfg(test)]
mod tests {

  mod parse {
    use crate::applications;
    use crate::cli::{Command, parse};

    #[test]
    fn no_arguments() {
      let apps = applications::all();
      let args = vec![].into_iter();
      let have = parse(args, &apps);
      let want = Ok(Command::DisplayHelp);
      pretty::assert_eq!(have, want);
    }

    mod rta_arguments {
      use crate::applications;
      use crate::cli::{Command, parse};
      use crate::commands::run;
      use crate::configuration::Version;
      use crate::error::UserError;
      use big_s::S;

      mod available {
        use crate::applications;
        use crate::cli::{Command, parse};
        use crate::commands::available;
        use crate::error::UserError;
        use big_s::S;

        #[test]
        fn with_app() {
          let apps = applications::all();
          let shellcheck = apps.lookup("shellcheck").unwrap();
          let args = vec![S("--available"), S("shellcheck")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::Available(available::Args {
            app_name: shellcheck.name(),
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
          let args = vec![S("--available"), S("--verbose"), S("shellcheck")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::Available(available::Args {
            app_name: shellcheck.name(),
            optional: false,
            version: None,
            verbose: true,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn without_app() {
          let apps = applications::all();
          let args = vec![S("--available")].into_iter();
          let have = parse(args, &apps);
          let want = Err(UserError::MissingApplication);
          pretty::assert_eq!(have, want);
        }
      }

      mod error_on_output {
        use crate::applications;
        use crate::cli::{Command, parse};
        use crate::commands::run;
        use crate::error::UserError;
        use big_s::S;

        #[test]
        fn normal() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let args = vec![S("--error-on-output"), S("actionlint")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::RunApp(run::Args {
            app_name: actionlint.name(),
            version: None,
            app_args: vec![],
            error_on_output: true,
            from_source: false,
            include_apps: vec![],
            optional: false,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_app() {
          let apps = applications::all();
          let args = vec![S("--error-on-output")].into_iter();
          let have = parse(args, &apps);
          let want = Err(UserError::MissingApplication);
          pretty::assert_eq!(have, want);
        }
      }

      mod install {
        use crate::applications;
        use crate::cli::{Command, parse};
        use crate::commands::install;
        use crate::error::UserError;
        use big_s::S;

        #[test]
        fn normal() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let args = vec![S("--install"), S("actionlint")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::Install(install::Args {
            app_name: actionlint.name(),
            version: None,
            from_source: false,
            include_apps: vec![],
            optional: false,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_app() {
          let apps = applications::all();
          let args = vec![S("--install")].into_iter();
          let have = parse(args, &apps);
          let want = Err(UserError::MissingApplication);
          pretty::assert_eq!(have, want);
        }
      }

      mod from_source {
        use crate::cli::parse;
        use crate::commands::run;
        use crate::{Command, applications};
        use big_s::S;

        #[test]
        fn flag() {
          let apps = applications::all();
          let args = vec![S("--from-source"), S("actionlint")].into_iter();
          let have = parse(args, &apps);
          let actionlint = apps.lookup("actionlint").unwrap();
          let want = Ok(Command::RunApp(run::Args {
            app_name: actionlint.name(),
            version: None,
            app_args: vec![],
            error_on_output: false,
            from_source: true,
            include_apps: vec![],
            optional: false,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }
      }

      mod test {
        use crate::applications;
        use crate::cli::{Command, parse};
        use crate::commands::test;
        use big_s::S;

        #[test]
        fn no_app_no_verbose() {
          let apps = applications::all();
          let args = vec![S("--test")].into_iter();
          let have = parse(args, &apps);
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
          let args = vec![S("--test"), S("--verbose")].into_iter();
          let have = parse(args, &apps);
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
          let args = vec![S("--test"), S("actionlint")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::Test(test::Args {
            optional: false,
            start_at_app: Some(actionlint.name()),
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn app_verbose() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let args = vec![S("--test"), S("--verbose"), S("actionlint")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::Test(test::Args {
            optional: false,
            start_at_app: Some(actionlint.name()),
            verbose: true,
          }));
          pretty::assert_eq!(have, want);
        }
      }

      mod help_parameter {
        use crate::applications;
        use crate::cli::{Command, parse};
        use big_s::S;

        #[test]
        fn short() {
          let apps = applications::all();
          let args = vec![S("-h")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::DisplayHelp);
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn long() {
          let apps = applications::all();
          let args = vec![S("--help")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::DisplayHelp);
          pretty::assert_eq!(have, want);
        }
      }

      mod include_apps {
        use crate::cli::parse;
        use crate::commands::run;
        use crate::configuration::Version;
        use crate::{Command, UserError, applications};
        use big_s::S;

        #[test]
        fn valid() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let gh = apps.lookup("gh").unwrap();
          let args = vec![S("--include=gh"), S("actionlint@2")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::RunApp(run::Args {
            app_name: actionlint.name(),
            version: Some(Version::from("2")),
            app_args: vec![],
            error_on_output: false,
            from_source: false,
            include_apps: vec![gh.name()],
            optional: false,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn invalid() {
          let apps = applications::all();
          let args = vec![S("--include=zonk"), S("actionlint@2")].into_iter();
          let have = parse(args, &apps);
          let want = Err(UserError::UnknownApp(S("zonk")));
          pretty::assert_eq!(have, want);
        }
      }

      mod verbose {
        use crate::applications;
        use crate::cli::{Command, parse};
        use crate::commands::run;
        use crate::configuration::Version;
        use crate::error::UserError;
        use big_s::S;

        #[test]
        fn long() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let args = vec![S("--verbose"), S("actionlint@2")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::RunApp(run::Args {
            app_name: actionlint.name(),
            version: Some(Version::from("2")),
            app_args: vec![],
            error_on_output: false,
            from_source: false,
            include_apps: vec![],
            optional: false,
            verbose: true,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn short() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let args = vec![S("-v"), S("actionlint@2")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::RunApp(run::Args {
            app_name: actionlint.name(),
            version: Some(Version::from("2")),
            app_args: vec![],
            error_on_output: false,
            from_source: false,
            include_apps: vec![],
            optional: false,
            verbose: true,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_app() {
          let apps = applications::all();
          let args = vec![S("--verbose")].into_iter();
          let have = parse(args, &apps);
          let want = Err(UserError::MissingApplication);
          pretty::assert_eq!(have, want);
        }
      }

      #[test]
      fn multiple_commands() {
        let apps = applications::all();
        let args = vec![S("--which"), S("--available"), S("shellcheck")].into_iter();
        let have = parse(args, &apps);
        let want = Err(UserError::MultipleCommandsGiven);
        pretty::assert_eq!(have, want);
      }

      #[test]
      fn optional() {
        let apps = applications::all();
        let actionlint = apps.lookup("actionlint").unwrap();
        let args = vec![S("--optional"), S("actionlint@2"), S("arg1")].into_iter();
        let have = parse(args, &apps);
        let want = Ok(Command::RunApp(run::Args {
          app_name: actionlint.name(),
          version: Some(Version::from("2")),
          app_args: vec![S("arg1")],
          error_on_output: false,
          from_source: false,
          include_apps: vec![],
          optional: true,
          verbose: false,
        }));
        pretty::assert_eq!(have, want);
      }

      mod version {
        use crate::applications;
        use crate::cli::{Command, parse};
        use big_s::S;

        #[test]
        fn short() {
          let apps = applications::all();
          let args = vec![S("-V")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::Version);
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn long() {
          let apps = applications::all();
          let args = vec![S("--version")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::Version);
          pretty::assert_eq!(have, want);
        }
      }

      mod versions {
        use crate::applications;
        use crate::cli::{Command, parse};
        use crate::commands::versions;
        use big_s::S;

        #[test]
        fn correct_usage() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let args = vec![S("--versions"), S("actionlint")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::Versions(versions::Args {
            app_name: actionlint.name(),
            amount: 10,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn custom_amount() {
          let apps = applications::all();
          let actionlint = apps.lookup("actionlint").unwrap();
          let args = vec![S("--versions=20"), S("actionlint")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::Versions(versions::Args {
            app_name: actionlint.name(),
            amount: 20,
            verbose: false,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_app() {
          let apps = applications::all();
          let args = vec![S("--versions")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::DisplayHelp);
          pretty::assert_eq!(have, want);
        }
      }

      mod which {
        use crate::cli::{Command, parse};
        use crate::error::UserError;
        use crate::{applications, commands};
        use big_s::S;

        #[test]
        fn with_app() {
          let apps = applications::all();
          let shellcheck = apps.lookup("shellcheck").unwrap();
          let args = vec![S("--which"), S("shellcheck")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::Which(commands::which::Args {
            app_name: shellcheck.name(),
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
          let args = vec![S("--which"), S("--verbose"), S("shellcheck")].into_iter();
          let have = parse(args, &apps);
          let want = Ok(Command::Which(commands::which::Args {
            app_name: shellcheck.name(),
            optional: false,
            version: None,
            verbose: true,
          }));
          pretty::assert_eq!(have, want);
        }

        #[test]
        fn without_app() {
          let apps = applications::all();
          let args = vec![S("--which")].into_iter();
          let have = parse(args, &apps);
          let want = Err(UserError::MissingApplication);
          pretty::assert_eq!(have, want);
        }
      }
    }

    mod application_arguments {
      use crate::applications;
      use crate::cli::{Command, parse};
      use crate::commands::run;
      use crate::configuration::Version;
      use big_s::S;

      #[test]
      fn no_arguments() {
        let apps = applications::all();
        let actionlint = apps.lookup("actionlint").unwrap();
        let args = vec![S("actionlint@2")].into_iter();
        let have = parse(args, &apps);
        let want = Ok(Command::RunApp(run::Args {
          app_name: actionlint.name(),
          version: Some(Version::from("2")),
          app_args: vec![],
          error_on_output: false,
          from_source: false,
          include_apps: vec![],
          optional: false,
          verbose: false,
        }));
        pretty::assert_eq!(have, want);
      }

      #[test]
      fn some_arguments() {
        let apps = applications::all();
        let actionlint = apps.lookup("actionlint").unwrap();
        let args = vec![S("actionlint@2"), S("--arg1"), S("arg2")].into_iter();
        let have = parse(args, &apps);
        let want = Ok(Command::RunApp(run::Args {
          app_name: actionlint.name(),
          version: Some(Version::from("2")),
          app_args: vec![S("--arg1"), S("arg2")],
          error_on_output: false,
          from_source: false,
          include_apps: vec![],
          optional: false,
          verbose: false,
        }));
        pretty::assert_eq!(have, want);
      }
    }

    mod rta_and_app_arguments {
      use crate::applications;
      use crate::cli::{Command, parse};
      use crate::commands::run;
      use crate::configuration::Version;
      use big_s::S;

      #[test]
      fn rta_and_app_arguments() {
        let apps = applications::all();
        let actionlint = apps.lookup("actionlint").unwrap();
        let args = vec![S("--verbose"), S("actionlint@2"), S("--arg1"), S("arg2")].into_iter();
        let have = parse(args, &apps);
        let want = Ok(Command::RunApp(run::Args {
          app_name: actionlint.name(),
          version: Some(Version::from("2")),
          app_args: vec![S("--arg1"), S("arg2")],
          error_on_output: false,
          from_source: false,
          include_apps: vec![],
          optional: false,
          verbose: true,
        }));
        pretty::assert_eq!(have, want);
      }

      #[test]
      fn same_arguments_as_run_that_app() {
        let apps = applications::all();
        let actionlint = apps.lookup("actionlint").unwrap();
        let args = vec![S("actionlint@2"), S("--verbose"), S("--version")].into_iter();
        let have = parse(args, &apps);
        let want = Ok(Command::RunApp(run::Args {
          app_name: actionlint.name(),
          version: Some(Version::from("2")),
          app_args: vec![S("--verbose"), S("--version")],
          error_on_output: false,
          from_source: false,
          include_apps: vec![],
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
