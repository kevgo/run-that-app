//! all applications that run-this-app can run

mod actionlint;
mod alphavet;
mod depth;
mod dprint;
mod gh;
mod ghokin;
mod gofumpt;
mod golangci_lint;
mod goreleaser;
mod scc;
mod shellcheck;
mod shfmt;

use crate::error::UserError;
use crate::install::InstallationMethod;
use crate::platform::Platform;
use crate::yard::Yard;
use crate::Result;
use std::slice;

pub trait App {
    /// the name by which the user can select this application at the run-that-app CLI
    fn name(&self) -> &'static str;

    /// the filename of the executable that starts this app
    fn executable_filename(&self, platform: Platform) -> &'static str;

    /// link to the (human-readable) homepage of the app
    fn homepage(&self) -> &'static str;

    fn installation_methods(&self, version: &str, platform: Platform, yard: &Yard) -> Vec<Box<dyn InstallationMethod>>;
}

pub fn all() -> Apps {
    Apps {
        list: vec![
            Box::new(actionlint::ActionLint {}),
            Box::new(alphavet::Alphavet {}),
            Box::new(depth::Depth {}),
            Box::new(dprint::Dprint {}),
            Box::new(gh::Gh {}),
            Box::new(ghokin::Ghokin {}),
            Box::new(gofumpt::Gofumpt {}),
            Box::new(golangci_lint::GolangCiLint {}),
            Box::new(goreleaser::Goreleaser {}),
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
    pub fn iter(&self) -> slice::Iter<'_, Box<dyn App>> {
        self.list.iter()
    }

    pub fn lookup(self, name: &str) -> Result<Box<dyn App>> {
        for app in self.list {
            if app.name() == name {
                return Ok(app);
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
