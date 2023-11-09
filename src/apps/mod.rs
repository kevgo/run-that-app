//! all applications that run-this-app can run

mod dprint;

use crate::error::UserError;
use crate::hosting::Hoster;
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
    fn executable(&self) -> &'static str;
    fn hoster(&self) -> Box<dyn Hoster>;
    fn files_to_extract_from_archive(&self, version: &str) -> Vec<String>;
}

fn all_apps() -> Vec<Box<dyn App>> {
    vec![Box::new(Dprint {})]
}
