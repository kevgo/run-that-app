//! all applications that run-this-app can run

mod dprint;

use crate::detect::Platform;
use crate::error::UserError;
use crate::hosting::OnlineAsset;
use crate::Result;
use dprint::Dprint;

pub fn lookup(name: &str) -> Result<Box<dyn App>> {
    for app in all_apps() {
        if app.name() == name {
            return Ok(app);
        }
    }
    return Err(UserError::UnknownApp(name.to_string()));
}

pub trait App {
    /// the name by which the user can select this application
    fn name(&self) -> &'static str;

    /// the name of the executable that starts this app
    fn executable(&self, platform: &Platform) -> &'static str;

    /// link to the (human-readable) homepage of the app
    fn homepage(&self) -> &'static str;

    /// the location at which the app is hosted online
    fn online_location(&self, version: String, platform: &Platform) -> Box<dyn OnlineAsset>;

    /// the name of the executable file in the archive
    fn file_to_extract_from_archive(&self, version: &str, platform: &Platform) -> String;
}

fn all_apps() -> Vec<Box<dyn App>> {
    vec![Box::new(Dprint {})]
}
