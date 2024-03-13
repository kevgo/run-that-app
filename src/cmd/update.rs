use crate::apps;
use crate::config::Config;
use crate::output::{Event, Output};
use crate::Result;
use std::process::ExitCode;

pub fn update(verbose: bool) -> Result<ExitCode> {
    let all_apps = apps::all();
    let mut config = Config::load(&all_apps)?;
    let output = Output { verbose };
    for old_app in &mut config.apps {
        let app = all_apps.lookup(&old_app.app)?;
        output.log(Event::UpdateBegin { app: &old_app.app });
        let latest = app.latest_installable_version(output)?;
        if let Some(previous) = &old_app.versions.update_largest_with(&latest) {
            output.log(Event::UpdateNewVersion {
                app: &old_app.app,
                old_version: previous,
                new_version: &latest,
            });
        } else {
            output.log(Event::UpdateAlreadyNewest { app: &old_app.app });
        }
    }
    config.save()?;
    Ok(ExitCode::SUCCESS)
}
