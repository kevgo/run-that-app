use crate::cmd::run::load_or_install;
use crate::config::{AppName, RequestedVersion};
use crate::output::Output;
use crate::platform::Platform;
use crate::{apps, Result};

pub trait OtherAppFolder {
    fn app_to_install(&self) -> AppName {
        // TODO: return Box::new(Go {}) here to link to the type directly and avoid stringly-typed code
        AppName::from("Go")
    }
}

pub fn install_other_app(app: &dyn OtherAppFolder, version: RequestedVersion, platform: Platform, output: &dyn Output) -> Result<bool> {
    let app_name = app.app_to_install();
    let all_apps = apps::all();
    let app = all_apps.lookup(&app_name)?;
    load_or_install(app, version, platform, output)?;
    Ok(true)
}
