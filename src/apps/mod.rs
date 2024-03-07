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
use crate::platform::Platform;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};
use std::slice::Iter;

pub trait App {
    /// the name by which the user can select this application at the run-that-app CLI
    fn name(&self) -> AppName;

    /// the filename of the executable that starts this app
    fn executable_filename(&self, platform: Platform) -> &'static str;

    /// relative path of the executable that starts this app in the folder the downloaded artifact gets unpacked into
    fn executable_filepath(&self, platform: Platform) -> &'static str;

    /// link to the (human-readable) homepage of the app
    fn homepage(&self) -> &'static str;

    /// installs this app at the given version into the given yard
    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>>;

    // loads this app from the given yard if it is already installed
    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable>;

    /// provides the versions of this application that can be installed
    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>>;

    /// provides the latest version of this application
    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version>;

    /// ensures that the given executable belongs to this app and if yes returns the installed version
    fn analyze_executable(&self, path: &Executable) -> AnalyzeResult;

    fn allowed_versions(&self) -> Option<semver::VersionReq>;
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
