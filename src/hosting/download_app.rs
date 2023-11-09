use crate::detect::Platform;
use crate::ui::RequestedApp;
use crate::yard::RunnableApp;
use crate::Result;
use std::path::PathBuf;

pub fn download_app(
    app: &RequestedApp,
    platform: &Platform,
    target: PathBuf,
) -> Result<RunnableApp> {
    Ok(RunnableApp {})
}
