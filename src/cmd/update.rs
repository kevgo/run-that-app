use crate::cli::RequestedApp;
use crate::output::Output;
use crate::Result;
use crate::{apps, config};
use std::process::ExitCode;

pub fn update(output: &dyn Output) -> Result<ExitCode> {
    output.println("updating");
    let config = config::load()?;
    let mut new_config: Vec<RequestedApp> = vec![];
    let all_apps = apps::all();
    for old_app in &config.apps {
        let app = all_apps.lookup(&old_app.name)?;
        output.println(&format!("updating {} ...", app.name()));
        let versions = app.versions(1, output)?;
        new_config.push(RequestedApp {
            name: app.name().to_string(),
            version: versions.into_iter().nth(0).unwrap_or(old_app.version.to_string()),
        })
    }
    // write new_config to disk
    Ok(ExitCode::SUCCESS)
}
