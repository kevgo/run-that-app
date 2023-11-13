//! all applications that run-this-app can run

mod alphavet;
mod depth;
mod dprint;
mod gh;
mod gofumpt;
mod golangci_lint;
mod scc;
mod shellcheck;
mod shfmt;

use crate::detect::Platform;
use crate::error::UserError;
use crate::hosting::OnlineLocation;
use crate::Result;

pub fn lookup(name: &str) -> Result<Box<dyn App>> {
    for app in all() {
        if app.name() == name {
            return Ok(app);
        }
    }
    Err(UserError::UnknownApp(name.to_string()))
}

pub trait App {
    /// the name by which the user can select this application at the run-that-app CLI
    fn name(&self) -> &'static str;

    /// the filename of the executable that starts this app
    fn executable(&self, platform: Platform) -> &'static str;

    /// link to the (human-readable) homepage of the app
    fn homepage(&self) -> &'static str;

    /// the location at which the app is hosted online
    fn artifact_location(&self, version: &str, platform: Platform) -> Box<dyn OnlineLocation>;

    /// the name of the executable file in the archive
    fn file_to_extract_from_archive(&self, version: &str, platform: Platform) -> Option<String>;
}

pub fn all() -> Vec<Box<dyn App>> {
    vec![
        Box::new(alphavet::Alphavet {}),
        Box::new(depth::Depth {}),
        Box::new(dprint::Dprint {}),
        Box::new(gh::Gh {}),
        Box::new(gofumpt::Gofumpt {}),
        Box::new(golangci_lint::GolangCiLint {}),
        Box::new(scc::Scc {}),
        Box::new(shellcheck::ShellCheck {}),
        Box::new(shfmt::Shfmt {}),
    ]
}
