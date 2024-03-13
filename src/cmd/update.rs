use crate::apps;
use crate::config::Config;
use crate::output::{self, Event};
use crate::Result;
use std::process::ExitCode;

pub fn update(verbose: bool) -> Result<ExitCode> {
    let all_apps = apps::all();
    let mut config = Config::load(&all_apps)?;
    let output = output::new(verbose);
    for old_app in &mut config.apps {
        let app = all_apps.lookup(&old_app.app)?;
        output(Event::UpdateBegin { app: &old_app.app });
        let latest = app.latest_installable_version(output)?;
        if let Some(previous) = &old_app.versions.update_largest_with(&latest) {
            output(Event::UpdateNewVersion {
                old_version: previous,
                new_version: &latest,
            });
        } else {
            output(Event::UpdateAlreadyNewest { app: &old_app.app });
        }
    }
    config.save()?;
    Ok(ExitCode::SUCCESS)
}
