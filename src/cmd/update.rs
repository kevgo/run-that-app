use crate::cli::RequestedApp;
use crate::config::Config;
use crate::output::Output;
use crate::Result;
use crate::{apps, config};
use colored::Colorize;
use std::process::ExitCode;

pub fn update(output: &dyn Output) -> Result<ExitCode> {
    let old_config = config::load()?;
    let mut new_config = Config::default();
    let all_apps = apps::all();
    for old_app in &old_config.apps {
        let app = all_apps.lookup(&old_app.name)?;
        output.print(&format!("updating {} ... ", old_app.name));
        let versions = app.versions(1, output)?;
        let new_version = versions.into_iter().next().unwrap_or_else(|| old_app.version.clone());
        if new_version == old_app.version {
            output.println(&format!("{}", "same".green()));
        } else {
            output.println(&format!("{} -> {}", old_app.version.cyan(), new_version.cyan()));
        }
        new_config.apps.push(RequestedApp {
            name: old_app.name.to_string(),
            version: new_version,
        });
    }
    config::save(&new_config)?;
    Ok(ExitCode::SUCCESS)
}
