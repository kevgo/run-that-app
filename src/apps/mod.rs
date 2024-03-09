//! all applications that run-this-app can run

mod actionlint;
mod alphavet;
mod deadcode;
mod depth;
mod dprint;
mod gh;
mod ghokin;
mod go;
mod goda;
mod gofmt;
mod gofumpt;
mod golangci_lint;
mod goreleaser;
mod mdbook;
mod nodejs;
mod npm;
mod npx;
mod scc;
mod shellcheck;
mod shfmt;

use crate::config::{AppName, Version};
use crate::error::UserError;
use crate::platform::{Os, Platform};
use crate::subshell::Executable;
use crate::{Output, Result};
use std::path::Path;
use std::slice::Iter;

pub trait App {
    /// the name by which the user can select this application at the run-that-app CLI
    fn name(&self) -> AppName;

    /// the filename of the executable that starts this app
    fn executable_filename(&self, platform: Platform) -> String {
        let bare = self.name().to_string();
        match platform.os {
            Os::Linux | Os::MacOS => bare,
            Os::Windows => format!("{bare}.exe"),
        }
    }

    /// which yard folder this app uses
    ///
    /// Apps can overwrite this method if they use the yard folder of another app.
    /// An example is npm. It's executable is located inside the yard folder of the Node app.
    fn yard_app(&self) -> AppName {
        self.name()
    }

    /// relative paths to the executable within the Yard folder
    ///
    /// By default, apps use the executable filename.
    /// Apps can override this method to provide additional custom paths.
    fn executable_locations(&self, platform: Platform) -> Vec<String> {
        vec![self.executable_filename(platform)]
    }

    /// link to the (human-readable) homepage of the app
    fn homepage(&self) -> &'static str;

    /// Tries to install this app at the given version into the given folder.
    /// Indicates whether a suitable installation method was found.
    fn install(&self, version: &Version, platform: Platform, folder: &Path, output: &dyn Output) -> Result<bool>;

    /// provides the versions of this application that can be installed
    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>>;

    /// provides the latest version of this application
    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version>;

    /// ensures that the given executable belongs to this app and if yes returns the installed version
    fn analyze_executable(&self, path: &Executable) -> AnalyzeResult;

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

pub enum AnalyzeResult {
    /// the given executable does not belong to this app
    NotIdentified,

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
        Box::new(ghokin::Ghokin {}),
        Box::new(go::Go {}),
        Box::new(goda::Goda {}),
        Box::new(gofmt::Gofmt {}),
        Box::new(gofumpt::Gofumpt {}),
        Box::new(golangci_lint::GolangCiLint {}),
        Box::new(goreleaser::Goreleaser {}),
        Box::new(mdbook::MdBook {}),
        Box::new(nodejs::NodeJS {}),
        Box::new(npm::Npm {}),
        Box::new(npx::Npx {}),
        Box::new(scc::Scc {}),
        Box::new(shellcheck::ShellCheck {}),
        Box::new(shfmt::Shfmt {}),
    ])
}

pub struct Apps(Vec<Box<dyn App>>);

impl Apps {
    /// provides an `Iterator` over the applications
    pub fn iter(&self) -> Iter<'_, Box<dyn App>> {
        self.0.iter()
    }

    /// provides the app with the given name
    pub fn lookup(&self, name: &AppName) -> Result<&dyn App> {
        for app in &self.0 {
            if app.name() == name {
                return Ok(app.as_ref());
            }
        }
        Err(UserError::UnknownApp(name.to_string()))
    }

    /// provides the length of the name of the app with the longest name
    pub fn longest_name_length(&self) -> usize {
        self.iter().map(|app| app.name().as_str().len()).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    mod apps {
        use crate::apps::{actionlint, dprint, shellcheck, Apps};

        #[test]
        fn longest_name_length() {
            let apps = Apps(vec![Box::new(dprint::Dprint {}), Box::new(actionlint::ActionLint {}), Box::new(shellcheck::ShellCheck {})]);
            let have = apps.longest_name_length();
            assert_eq!(have, 10);
        }

        mod lookup {
            use crate::apps::{dprint, shellcheck, Apps};
            use crate::config::AppName;
            use crate::UserError;
            use big_s::S;

            #[test]
            fn known_app() {
                let apps = Apps(vec![Box::new(dprint::Dprint {}), Box::new(shellcheck::ShellCheck {})]);
                let shellcheck = AppName::from("shellcheck");
                let have = apps.lookup(&shellcheck).unwrap();
                assert_eq!(have.name(), &shellcheck);
            }

            #[test]
            fn unknown_app() {
                let apps = Apps(vec![Box::new(dprint::Dprint {}), Box::new(shellcheck::ShellCheck {})]);
                let Err(err) = apps.lookup(&AppName::from("zonk")) else {
                    panic!("expected an error here");
                };
                assert_eq!(err, UserError::UnknownApp(S("zonk")));
            }
        }
    }
}
