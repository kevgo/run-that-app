use crate::cmd::run::load_or_install;
use crate::config::{AppName, RequestedVersion, Version};
use crate::output::Output;
use crate::platform::Platform;
use crate::{apps, Result};

pub trait OtherAppFolder {
    fn app_to_install(&self) -> AppName {
        // TODO: return Box::new(Go {}) here to link to the type directly and avoid stringly-typed code
        AppName::from("Go")
    }
}

pub fn install_other_app(app: &dyn OtherAppFolder, version: Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    let app_name = app.app_to_install();
    let all_apps = apps::all();
    let app = all_apps.lookup(&app_name)?;
    // Note: we know it must be the Yard variant here. At this point we are installing the app.
    // Only Yard variants get installed. The Path variant doesn't get installed.
    load_or_install(app, RequestedVersion::Yard(version), platform, output)?;
    Ok(true)
}
