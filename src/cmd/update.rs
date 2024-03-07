use crate::config::Config;
use crate::output::Output;
use crate::Result;
use crate::{apps, output};
use colored::Colorize;
use std::process::ExitCode;

pub fn update(log: Option<String>) -> Result<ExitCode> {
    let mut config = Config::load()?;
    let all_apps = apps::all();
    let output = output::StdErr { category: log };
    for old_app in &mut config.apps {
        let app = all_apps.lookup(&old_app.app)?;
        output.print(&format!("updating {} ... ", &old_app.app));
        let latest = app.latest_installable_version(&output)?;
        let latest_str = latest.to_string();
        if let Some(previous) = &old_app.versions.update_largest_with(&latest) {
            output.println(&format!("{} -> {}", previous.as_str().cyan(), latest_str.cyan()));
        } else {
            output.println(&format!("{}", "current".green()));
        }
    }
    config.save()?;
    Ok(ExitCode::SUCCESS)
}
