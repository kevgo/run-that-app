//! all applications that run-this-app can run

mod actionlint;
mod alphavet;
mod deadcode;
mod depth;
mod dprint;
mod exhaustruct;
mod gh;
mod ghokin;
pub mod go;
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

use crate::configuration::{ApplicationName, Version};
use crate::platform::Platform;
use crate::prelude::*;
use crate::run::{self, ExecutableNameUnix, ExecutablePath};
use crate::Log;
use std::fmt::Display;
use std::slice::Iter;

pub trait App {
  /// the name by which the user can select this application at the run-that-app CLI
  fn name(&self) -> ApplicationName;

  /// the filename of the executable that starts this app
  fn default_executable_filename(&self) -> ExecutableNameUnix {
    ExecutableNameUnix::from(self.name().inner())
  }

  /// names of other executables that this app provides
  fn additional_executables(&self) -> Vec<ExecutableNameUnix> {
    vec![]
  }

  /// the various ways to install and run this application
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
  fn clone(&self) -> Box<dyn App>;

  /// provides the app that contains the executable for this app,
  /// the name of the executable provided by this app to call,
  /// and arguments to call that executable with.
  fn carrier(&self, version: &Version, platform: Platform) -> (Box<dyn App>, ExecutableNameUnix, Vec<String>) {
    match self.run_method(version, platform) {
      run::Method::ThisApp { install_methods: _ } => (self.clone(), self.default_executable_filename(), vec![]),
      run::Method::OtherAppOtherExecutable { app, executable_name } => (app.clone(), executable_name, vec![]),
      run::Method::OtherAppDefaultExecutable { app, args } => (app.clone(), app.default_executable_filename(), args),
    }
  }
}

impl Display for dyn App {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.name().as_str())
  }
}

pub enum AnalyzeResult {
  /// the given executable does not belong to this app
  NotIdentified { output: String },

  /// the given executable belongs to this app but doesn't allow determining the version
  IdentifiedButUnknownVersion,

  /// the given executable belongs to this app and has the contained version
  IdentifiedWithVersion(Version),
}

pub fn all() -> Apps {
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

pub struct Apps(Vec<Box<dyn App>>);

impl Apps {
  /// provides an `Iterator` over the applications
  pub fn iter(&self) -> Iter<'_, Box<dyn App>> {
    self.0.iter()
  }

  /// provides the app with the given name
  /// TODO: return the actual Box<dyn App> instead of a reference here
  pub fn lookup(&self, name: &ApplicationName) -> Result<&dyn App> {
    for app in &self.0 {
      if app.name() == name {
        return Ok(app.as_ref());
      }
    }
    Err(UserError::UnknownApp(name.to_string()))
  }

  /// provides the length of the name of the app with the longest name
  pub fn longest_name_length(&self) -> usize {
    self.iter().map(|app| app.name().as_str().len()).max().unwrap_or_default()
  }
}

impl IntoIterator for Apps {
  type Item = Box<dyn App>;
  type IntoIter = std::vec::IntoIter<Box<dyn App>>;

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
      use crate::applications::{dprint, shellcheck, Apps};
      use crate::configuration::ApplicationName;
      use crate::prelude::*;
      use big_s::S;

      #[test]
      fn known_app() {
        let apps = Apps(vec![Box::new(dprint::Dprint {}), Box::new(shellcheck::ShellCheck {})]);
        let shellcheck = ApplicationName::from("shellcheck");
        let have = apps.lookup(&shellcheck).unwrap();
        assert_eq!(have.name(), &shellcheck);
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
