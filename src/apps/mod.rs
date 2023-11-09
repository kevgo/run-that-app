//! apps that run-this-app knows how to execute

use crate::error::UserError;
use crate::ui::{Output, RequestedApp};
use crate::Platform;
use crate::Result;

pub fn lookup(name: &str) -> Result<Box<dyn App>> {
    for app in all_apps() {
        if app.name() == name {
            return Ok(app);
        }
    }
    return Err(UserError::UnknownApp(name.to_string()));
}

pub trait App {
    fn name(&self) -> &'static str;
    fn repo(&self) -> &'static str;
    fn install(&self, request: &RequestedApp, platform: Platform, output: &Output) -> Result<()>;
}

fn all_apps() -> Vec<Box<dyn App>> {
    vec![]
}
