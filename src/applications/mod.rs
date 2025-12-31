//! all applications that run-this-app can run

mod actionlint;
mod alphavet;
mod contest;
mod cucumber_sort;
mod deadcode;
mod depth;
mod dprint;
mod exhaustruct;
mod funcorder;
mod gh;
mod ghokin;
pub(crate) mod go;
mod goda;
mod gofmt;
mod gofumpt;
mod golangci_lint;
mod goreleaser;
mod govulnchec;
mod ireturn;
mod keep_sorted;
mod mdbook;
mod mdbook_linkcheck;
mod node_prune;
mod nodejs;
mod npm;
mod npx;
mod rclone;
mod ripgrep;
mod scc;
mod shellcheck;
mod shfmt;
mod staticcheck;
mod tikibase;

use crate::Log;
use crate::configuration::Version;
use crate::error::{Result, UserError};
use crate::executables::{Executable, ExecutableArgs, ExecutableNameUnix, RunMethod};
use crate::platform::Platform;
use dyn_clone::DynClone;
use std::fmt::{Debug, Display};
use std::path::Path;

pub(crate) fn all() -> Apps {
  Apps(vec![
    Box::new(actionlint::ActionLint {}),
    Box::new(alphavet::Alphavet {}),
    Box::new(contest::Contest {}),
    Box::new(cucumber_sort::CucumberSort {}),
    Box::new(deadcode::Deadcode {}),
    Box::new(depth::Depth {}),
    Box::new(dprint::Dprint {}),
    Box::new(gh::Gh {}),
    Box::new(exhaustruct::Exhaustruct {}),
    Box::new(funcorder::FuncOrder {}),
    Box::new(ghokin::Ghokin {}),
    Box::new(go::Go {}),
    Box::new(goda::Goda {}),
    Box::new(gofmt::Gofmt {}),
    Box::new(gofumpt::Gofumpt {}),
    Box::new(golangci_lint::GolangCiLint {}),
    Box::new(goreleaser::Goreleaser {}),
    Box::new(govulnchec::Govulncheck {}),
    Box::new(ireturn::Ireturn {}),
    Box::new(keep_sorted::KeepSorted {}),
    Box::new(mdbook::MdBook {}),
    Box::new(mdbook_linkcheck::MdBookLinkCheck {}),
    Box::new(nodejs::NodeJS {}),
    Box::new(node_prune::NodePrune {}),
    Box::new(npm::Npm {}),
    Box::new(npx::Npx {}),
    Box::new(ripgrep::RipGrep {}),
    Box::new(rclone::Rclone {}),
    Box::new(scc::Scc {}),
    Box::new(shellcheck::ShellCheck {}),
    Box::new(shfmt::Shfmt {}),
    Box::new(staticcheck::StaticCheck {}),
    Box::new(tikibase::Tikibase {}),
  ])
}

/// all the information about an application that run-that-app can install
pub(crate) trait AppDefinition: DynClone {
  /// the name by which the user can select this application at the run-that-app CLI
  fn name(&self) -> ApplicationName;

  /// the filename of the executable that starts this app
  fn executable_filename(&self) -> ExecutableNameUnix {
    ExecutableNameUnix::from(self.name())
  }

  /// names of other executables that this app provides
  fn additional_executables(&self) -> Vec<ExecutableNameUnix> {
    vec![]
  }

  /// define how to run this application
  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod;

  /// link to the (human-readable) homepage of the app
  fn homepage(&self) -> &'static str;

  /// provides the versions of this application that can be installed
  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>>;

  /// provides the latest version of this application that can be installed
  fn latest_installable_version(&self, log: Log) -> Result<Version>;

  /// ensures that the given executable belongs to this app and if yes returns its version
  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult>;

  /// Apps can override this method to provide version restrictions
  /// defined by config files in the working directory.
  /// Apps that don't override this method are considered
  /// to have no such version restrictions.
  ///
  /// Examples: in a Go codebase, a file "go.mod" might define
  /// which Go version to use to compile this codebase
  /// Similar version restrictions can exist in
  /// "package.json" for `NodeJS` or "Gemfile" for Ruby.
  fn allowed_versions(&self) -> Result<semver::VersionReq> {
    Ok(semver::VersionReq::STAR)
  }
}

dyn_clone::clone_trait_object!(AppDefinition);

/// provides the app that contains the executable for the given app,
/// the name of the executable provided by this app to call,
/// and arguments to call that executable with.
pub(crate) fn carrier<'a>(
  app: &'a dyn AppDefinition,
  version: &Version,
  platform: Platform,
) -> (Box<dyn AppDefinition + 'a>, ExecutableNameUnix, ExecutableArgs) {
  match app.run_method(version, platform) {
    RunMethod::ThisApp { install_methods: _ } => (dyn_clone::clone_box(app), app.executable_filename(), ExecutableArgs::None),
    RunMethod::OtherAppOtherExecutable {
      app_definition,
      executable_name,
    } => (dyn_clone::clone_box(app_definition.as_ref()), executable_name, ExecutableArgs::None),
    RunMethod::OtherAppDefaultExecutable { app_definition, args } => {
      (dyn_clone::clone_box(app_definition.as_ref()), app_definition.executable_filename(), args)
    }
  }
}

impl PartialEq for dyn AppDefinition {
  fn eq(&self, other: &Self) -> bool {
    self.name() == other.name()
  }
}

impl Debug for dyn AppDefinition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.name().as_ref())
  }
}

#[derive(Debug, PartialEq)]
pub(crate) struct ApplicationName(&'static str);

impl ApplicationName {
  pub(crate) fn as_str(&self) -> &str {
    self.0
  }

  pub(crate) fn len(&self) -> usize {
    self.0.len()
  }
}

impl From<&'static str> for ApplicationName {
  fn from(value: &'static str) -> Self {
    ApplicationName(value)
  }
}

impl Display for ApplicationName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.0)
  }
}

impl AsRef<Path> for ApplicationName {
  fn as_ref(&self) -> &Path {
    Path::new(&self.0)
  }
}

impl AsRef<str> for ApplicationName {
  fn as_ref(&self) -> &str {
    self.0
  }
}

impl PartialEq<&str> for ApplicationName {
  fn eq(&self, other: &&str) -> bool {
    self.0 == *other
  }
}

pub(crate) enum AnalyzeResult {
  /// the given executable does not belong to this app
  NotIdentified { output: String },

  /// the given executable belongs to this app but doesn't allow determining the version
  IdentifiedButUnknownVersion,

  /// the given executable belongs to this app and has the contained version
  IdentifiedWithVersion(Version),
}

pub(crate) struct Apps(Vec<Box<dyn AppDefinition>>);

impl Apps {
  /// provides the app with the given name
  pub(crate) fn lookup<AS: AsRef<str>>(&self, name: AS) -> Result<&dyn AppDefinition> {
    for app in &self.0 {
      if app.name() == name.as_ref() {
        return Ok(app.as_ref());
      }
      if app.executable_filename().as_ref() == name.as_ref() {
        return Ok(app.as_ref());
      }
    }
    Err(UserError::UnknownApp(name.as_ref().to_string()))
  }

  /// provides the length of the name of the app with the longest name
  pub(crate) fn longest_name_length(&self) -> usize {
    self.into_iter().map(|app| app.name().len()).max().unwrap_or_default()
  }
}

impl IntoIterator for Apps {
  type Item = Box<dyn AppDefinition>;
  type IntoIter = std::vec::IntoIter<Box<dyn AppDefinition>>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a Apps {
  type Item = &'a Box<dyn AppDefinition + 'a>;
  type IntoIter = std::slice::Iter<'a, Box<dyn AppDefinition + 'a>>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

#[cfg(test)]
mod tests {
  mod apps {
    use crate::applications::{Apps, actionlint, dprint, shellcheck};

    #[test]
    fn longest_name_length() {
      let apps = Apps(vec![
        Box::new(dprint::Dprint {}),
        Box::new(actionlint::ActionLint {}),
        Box::new(shellcheck::ShellCheck {}),
      ]);
      let have = apps.longest_name_length();
      assert_eq!(have, 10);
    }

    mod lookup {
      use crate::applications::{Apps, dprint, shellcheck};
      use crate::error::UserError;
      use big_s::S;

      #[test]
      fn known_app() {
        let apps = Apps(vec![Box::new(dprint::Dprint {}), Box::new(shellcheck::ShellCheck {})]);
        let have = apps.lookup("shellcheck").unwrap();
        assert_eq!(have.name(), "shellcheck");
      }

      #[test]
      #[allow(clippy::panic)]
      fn unknown_app() {
        let apps = Apps(vec![Box::new(dprint::Dprint {}), Box::new(shellcheck::ShellCheck {})]);
        let Err(err) = apps.lookup("zonk") else {
          panic!("expected an error here");
        };
        assert_eq!(err, UserError::UnknownApp(S("zonk")));
      }
    }
  }
}
