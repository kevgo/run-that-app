//! all applications that run-this-app can run

mod dprint;
mod shellcheck;
mod shfmt;

use crate::detect::Platform;
use crate::error::UserError;
use crate::hosting::OnlineLocation;
use crate::Result;
use dprint::Dprint;
use shellcheck::ShellCheck;
use shfmt::Shfmt;

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
    fn file_to_extract_from_archive(&self, version: &str, platform: Platform) -> String;
}

pub fn all() -> Vec<Box<dyn App>> {
    vec![
        Box::new(Dprint {}),
        Box::new(ShellCheck {}),
        Box::new(Shfmt {}),
    ]
}
