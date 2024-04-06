use crate::apps;
use crate::config::Config;
use crate::logger::{self, Event};
use crate::prelude::*;
use std::process::ExitCode;

pub fn update(verbose: bool) -> Result<ExitCode> {
  let all_apps = apps::all();
  let mut config = Config::load(&all_apps)?;
  let log = logger::new(verbose);
  for old_app in &mut config.apps {
    let app = all_apps.lookup(&old_app.app)?;
    log(Event::UpdateBegin { app: &old_app.app });
    let latest = app.latest_installable_version(log)?;
    if let Some(previous) = &old_app.versions.update_largest_with(&latest) {
      log(Event::UpdateNewVersion {
        old_version: previous,
        new_version: &latest,
      });
    } else {
      log(Event::UpdateAlreadyNewest { app: &old_app.app });
    }
  }
  config.save()?;
  Ok(ExitCode::SUCCESS)
}
