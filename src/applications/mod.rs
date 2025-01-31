//! all applications that run-this-app can run

mod actionlint;
mod alphavet;
mod deadcode;
mod depth;
mod dprint;
mod exhaustruct;
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
mod mdbook;
mod mdbook_linkcheck;
mod node_prune;
mod nodejs;
mod npm;
mod npx;
mod scc;
mod shellcheck;
mod shfmt;
mod staticcheck;
mod tikibase;

use crate::configuration::Version;
use crate::platform::Platform;
use crate::prelude::*;
use crate::run::{self, ExecutableArgs, ExecutableNameUnix, ExecutablePath};
use crate::Log;
use std::fmt::{Debug, Display};
use std::path::Path;
use std::slice::Iter;

pub(crate) fn all() -> Apps {
  Apps(vec![
    Box::new(actionlint::ActionLint {}),
    Box::new(alphavet::Alphavet {}),
    Box::new(deadcode::Deadcode {}),
    Box::new(depth::Depth {}),
    Box::new(dprint::Dprint {}),
    Box::new(gh::Gh {}),
    Box::new(exhaustruct::Exhaustruct {}),
    Box::new(ghokin::Ghokin {}),
    Box::new(go::Go {}),
    Box::new(goda::Goda {}),
    Box::new(gofmt::Gofmt {}),
    Box::new(gofumpt::Gofumpt {}),
    Box::new(golangci_lint::GolangCiLint {}),
    Box::new(goreleaser::Goreleaser {}),
    Box::new(govulnchec::Govulncheck {}),
    Box::new(ireturn::Ireturn {}),
    Box::new(mdbook::MdBook {}),
    Box::new(mdbook_linkcheck::MdBookLinkCheck {}),
    Box::new(nodejs::NodeJS {}),
    Box::new(node_prune::NodePrune {}),
    Box::new(npm::Npm {}),
    Box::new(npx::Npx {}),
    Box::new(scc::Scc {}),
    Box::new(shellcheck::ShellCheck {}),
    Box::new(shfmt::Shfmt {}),
    Box::new(staticcheck::StaticCheck {}),
    Box::new(tikibase::Tikibase {}),
  ])
}

/// allows definining an application that run-that-app can install
pub(crate) trait AppDefinition {
  /// the name by which the user can select this application at the run-that-app CLI
  fn name(&self) -> &'static str;

  /// type-safe version of self.name, for internal use
  fn app_name(&self) -> ApplicationName {
    ApplicationName::from(self.name())
  }

  /// the filename of the executable that starts this app
  fn default_executable_filename(&self) -> ExecutableNameUnix {
    ExecutableNameUnix::from(self.name())
  }

  /// names of other executables that this app provides
  fn additional_executables(&self) -> Vec<ExecutableNameUnix> {
    vec![]
  }

  /// define how to run this application
  fn run_method(&self, version: &Version, platform: Platform) -> run::Method;

  /// link to the (human-readable) homepage of the app
  fn homepage(&self) -> &'static str;

  /// provides the versions of this application that can be installed
  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>>;

  /// provides the latest version of this application that can be installed
  fn latest_installable_version(&self, log: Log) -> Result<Version>;

  /// ensures that the given executable belongs to this app and if yes returns its version
  fn analyze_executable(&self, executable: &ExecutablePath, log: Log) -> Result<AnalyzeResult>;

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

  /// this is necessary because a limitation of Rust does not allow deriving the Clone trait automatically
  fn clone(&self) -> Box<dyn AppDefinition>;

  /// provides the app that contains the executable for this app,
  /// the name of the executable provided by this app to call,
  /// and arguments to call that executable with.
  fn carrier(&self, version: &Version, platform: Platform) -> (Box<dyn AppDefinition>, ExecutableNameUnix, ExecutableArgs) {
    match self.run_method(version, platform) {
      run::Method::ThisApp { install_methods: _ } => (self.clone(), self.default_executable_filename(), ExecutableArgs::None),
      run::Method::OtherAppOtherExecutable {
        app_definition,
        executable_name,
      } => (app_definition.clone(), executable_name, ExecutableArgs::None),
      run::Method::OtherAppDefaultExecutable { app_definition, args } => (app_definition.clone(), app_definition.default_executable_filename(), args),
    }
  }
}

impl Display for dyn AppDefinition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.name())
  }
}

impl PartialEq for dyn AppDefinition {
  fn eq(&self, other: &Self) -> bool {
    self.name() == other.name()
  }
}

impl Debug for dyn AppDefinition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.name())
  }
}

#[derive(Debug, PartialEq)]
pub(crate) struct ApplicationName(String);

impl ApplicationName {
  pub(crate) fn as_str(&self) -> &str {
    &self.0
  }

  pub(crate) fn new(name: String) -> ApplicationName {
    ApplicationName(name)
  }
}

impl From<&str> for ApplicationName {
  fn from(value: &str) -> Self {
    assert!(!value.is_empty(), "empty app name");
    assert!(value.to_lowercase() == value, "app name is not all lowercase");
    ApplicationName::new(value.to_string())
  }
}

impl Display for ApplicationName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl PartialEq<&str> for ApplicationName {
  fn eq(&self, other: &&str) -> bool {
    self.0 == *other
  }
}

impl PartialEq<&ApplicationName> for ApplicationName {
  fn eq(&self, other: &&ApplicationName) -> bool {
    self == *other
  }
}

impl AsRef<Path> for ApplicationName {
  fn as_ref(&self) -> &Path {
    Path::new(&self.0)
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
  /// provides an `Iterator` over the applications
  pub(crate) fn iter(&self) -> Iter<'_, Box<dyn AppDefinition>> {
    self.0.iter()
  }

  /// provides the app with the given name
  /// TODO: return the actual Box<dyn App> instead of a reference here
  pub(crate) fn lookup(&self, name: &ApplicationName) -> Result<&dyn AppDefinition> {
    for app in &self.0 {
      if app.name() == name.as_str() {
        return Ok(app.as_ref());
      }
    }
    Err(UserError::UnknownApp(name.to_string()))
  }

  /// provides the length of the name of the app with the longest name
  pub(crate) fn longest_name_length(&self) -> usize {
    self.iter().map(|app| app.name().len()).max().unwrap_or_default()
  }
}

impl IntoIterator for Apps {
  type Item = Box<dyn AppDefinition>;
  type IntoIter = std::vec::IntoIter<Box<dyn AppDefinition>>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

#[cfg(test)]
mod tests {
  mod apps {
    use crate::applications::{actionlint, dprint, shellcheck, Apps};

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
      use crate::applications::{dprint, shellcheck, ApplicationName, Apps};
      use crate::prelude::*;
      use big_s::S;

      #[test]
      fn known_app() {
        let apps = Apps(vec![Box::new(dprint::Dprint {}), Box::new(shellcheck::ShellCheck {})]);
        let shellcheck = ApplicationName::from("shellcheck");
        let have = apps.lookup(&shellcheck).unwrap();
        assert_eq!(have.name(), shellcheck.as_str());
      }

      #[test]
      #[allow(clippy::panic)]
      fn unknown_app() {
        let apps = Apps(vec![Box::new(dprint::Dprint {}), Box::new(shellcheck::ShellCheck {})]);
        let Err(err) = apps.lookup(&ApplicationName::from("zonk")) else {
          panic!("expected an error here");
        };
        assert_eq!(err, UserError::UnknownApp(S("zonk")));
      }
    }
  }
}
