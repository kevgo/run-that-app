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
mod nodejs;
mod npm;
mod npx;
mod scc;
mod shellcheck;
mod shfmt;

use crate::error::UserError;
use crate::platform::Platform;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};
use std::slice::Iter;

pub trait App {
    /// the name by which the user can select this application at the run-that-app CLI
    fn name(&self) -> &'static str;

    /// the filename of the executable that starts this app
    fn executable_filename(&self, platform: Platform) -> &'static str;

    /// link to the (human-readable) homepage of the app
    fn homepage(&self) -> &'static str;

    /// installs this app at the given version into the given yard
    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>>;

    // loads this app from the given yard if it is already installed
    fn load(&self, version: &str, platform: Platform, yard: &Yard) -> Option<Executable>;

    /// provides the versions of this application that can be installed
    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<String>>;

    /// provides the latest version of this application
    fn latest_installable_version(&self, output: &dyn Output) -> Result<String>;
}

pub fn all() -> Apps {
    Apps {
        list: vec![
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
            Box::new(nodejs::NodeJS {}),
            Box::new(npm::Npm {}),
            Box::new(npx::Npx {}),
            Box::new(scc::Scc {}),
            Box::new(shellcheck::ShellCheck {}),
            Box::new(shfmt::Shfmt {}),
        ],
    }
}

pub struct Apps {
    pub list: Vec<Box<dyn App>>,
}

impl Apps {
    /// provides an `Iterator` over the applications
    pub fn iter(&self) -> Iter<'_, Box<dyn App>> {
        self.list.iter()
    }

    /// provides the app with the given name
    pub fn lookup(&self, name: &str) -> Result<&dyn App> {
        for app in &self.list {
            if app.name() == name {
                return Ok(app.as_ref());
            }
        }
        Err(UserError::UnknownApp(name.to_string()))
    }

    /// provides the length of the name of the app with the longest name
    pub fn longest_name_length(&self) -> usize {
        self.iter().map(|app| app.name().len()).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    mod apps {
        use crate::apps::{actionlint, dprint, shellcheck, Apps};

        #[test]
        fn longest_name_length() {
            let apps = Apps {
                list: vec![
                    Box::new(dprint::Dprint {}),
                    Box::new(actionlint::ActionLint {}),
                    Box::new(shellcheck::ShellCheck {}),
                ],
            };
            let have = apps.longest_name_length();
            assert_eq!(have, 10);
        }

        mod lookup {
            use crate::apps::{dprint, shellcheck, Apps};
            use crate::UserError;
            use big_s::S;

            #[test]
            fn known_app() {
                let apps = Apps {
                    list: vec![Box::new(dprint::Dprint {}), Box::new(shellcheck::ShellCheck {})],
                };
                let have = apps.lookup("shellcheck").unwrap();
                assert_eq!(have.name(), "shellcheck");
            }

            #[test]
            fn unknown_app() {
                let apps = Apps {
                    list: vec![Box::new(dprint::Dprint {}), Box::new(shellcheck::ShellCheck {})],
                };
                let Err(err) = apps.lookup("zonk") else {
                    panic!("expected an error here");
                };
                assert_eq!(err, UserError::UnknownApp(S("zonk")));
            }
        }
    }
}
