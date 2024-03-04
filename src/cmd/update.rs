use crate::config::Config;
use crate::output::Output;
use crate::Result;
use crate::{apps, config};
use colored::Colorize;
use std::process::ExitCode;

pub fn update(output: &dyn Output) -> Result<ExitCode> {
    let old_config = Config::load()?;
    let all_apps = apps::all();
    for old_app in old_config.apps {
        let app = all_apps.lookup(&old_app.app)?;
        output.print(&format!("updating {} ... ", &old_app.app));
        let latest = app.latest_installable_version(output)?;
        if let Some(previous) = old_app.versions.update_largest_with(latest) {
            output.println(&format!("{} -> {}", previous.as_str().cyan(), latest.as_str().cyan()));
        } else {
            output.println(&format!("{}", "current".green()));
        }
    }
    config::save(&old_config)?;
    Ok(ExitCode::SUCCESS)
}
