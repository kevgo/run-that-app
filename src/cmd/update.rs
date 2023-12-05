use crate::cli::RequestedApp;
use crate::config::Config;
use crate::output::Output;
use crate::Result;
use crate::{apps, config};
use std::process::ExitCode;

pub fn update(output: &dyn Output) -> Result<ExitCode> {
    output.println("updating");
    let config = config::load()?;
    let mut new_config = Config::default();
    let all_apps = apps::all();
    for old_app in &config.apps {
        let app = all_apps.lookup(&old_app.name)?;
        output.println(&format!("updating {} ...", app.name()));
        let versions = app.versions(1, output)?;
        let new_version = versions.into_iter().next().unwrap_or_else(|| old_app.version.clone());
        new_config.apps.push(RequestedApp {
            name: old_app.name.to_string(),
            version: new_version,
        });
    }
    config::save(new_config)?;
    // write new_config to disk
    Ok(ExitCode::SUCCESS)
}
