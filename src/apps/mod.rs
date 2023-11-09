//! all applications that run-this-app can run

mod dprint;

use crate::detect::Platform;
use crate::error::UserError;
use crate::hosting::OnlineAsset;
use crate::Result;
use dprint::Dprint;

pub fn lookup(name: &str) -> Result<Box<dyn App>> {
    for app in all_apps() {
        if app.executable() == name {
            return Ok(app);
        }
    }
    return Err(UserError::UnknownApp(name.to_string()));
}

pub trait App {
    /// the name of the executable that starts this app
    fn executable(&self) -> &'static str;

    /// link to the homepage of the app
    fn homepage(&self) -> &'static str;

    /// downloads the app for the given version and platform into the given yard
    fn online_asset(&self, version: String, platform: &Platform) -> Box<dyn OnlineAsset>;

    fn file_to_extract_from_archive(&self, version: &str, platform: &Platform) -> String;
}

fn all_apps() -> Vec<Box<dyn App>> {
    vec![Box::new(Dprint {})]
}
