use crate::apps::App;
use crate::cmd::run::load_or_install;
use crate::config::{RequestedVersion, Version};
use crate::output::Output;
use crate::platform::Platform;
use crate::{apps, Result};

pub trait OtherAppFolder: App {
    fn app_to_install(&self) -> Box<dyn App>;
}

pub fn install_other_app(app: &dyn OtherAppFolder, version: &Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    let app_to_install = app.app_to_install();
    let all_apps = apps::all();
    let app = all_apps.lookup(&app_to_install.name())?;
    // Note: we know it must be the Yard variant here. At this point we are installing the app.
    // Only Yard variants get installed. The Path variant doesn't get installed.
    load_or_install(app, RequestedVersion::Yard(version.to_owned()), platform, output)?;
    Ok(true)
}
