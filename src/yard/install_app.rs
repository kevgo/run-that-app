use super::RunnableApp;
use crate::detect::Platform;
use crate::ui::RequestedApp;
use crate::Result;

pub fn install_app(requested_app: &RequestedApp, platform: &Platform) -> Result<RunnableApp> {
    Ok(RunnableApp {})
}
