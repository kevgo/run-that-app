use crate::config::{AppVersion, Config};
use crate::output::Output;
use crate::Result;
use crate::{apps, config};
use colored::Colorize;
use std::process::ExitCode;

pub fn update(output: &dyn Output) -> Result<ExitCode> {
    let old_config = config::load()?;
    let mut new_config = Config::default();
    let all_apps = apps::all();
    for old_app in old_config.apps {
        let app = all_apps.lookup(&old_app.name)?;
        output.print(&format!("updating {} ... ", &old_app.name));
        let latest = app.latest_installable_version(output)?;
        if old_app.version == latest {
            output.println(&format!("{}", "current".green()));
        } else {
            output.println(&format!("{} -> {}", &old_app.version.as_str().cyan(), latest.as_str().cyan()));
        }
        new_config.apps.push(AppVersion {
            name: old_app.name,
            version: latest,
        });
    }
    config::save(&new_config)?;
    Ok(ExitCode::SUCCESS)
}
