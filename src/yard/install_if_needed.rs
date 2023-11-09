use super::{install_app, load_runnable_app, RunnableApp};
use crate::detect::Platform;
use crate::ui::RequestedApp;
use crate::Result;

pub fn load_or_install(requested_app: RequestedApp, platform: Platform) -> Result<RunnableApp> {
    if let Some(runnable_app) = load_runnable_app(&requested_app) {
        Ok(runnable_app)
    } else {
        install_app(&requested_app, &platform)
    }
}
