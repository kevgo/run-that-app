use crate::applications;
use crate::configuration::File;
use crate::logging::{self, Event};
use crate::prelude::*;
use std::process::ExitCode;

pub(crate) fn update(args: &Args) -> Result<ExitCode> {
  let all_apps = applications::all();
  let mut config = File::load(&all_apps)?;
  let log = logging::new(args.verbose);
  for old_app in &mut config.apps {
    let app = all_apps.lookup(&old_app.app_name)?;
    log(Event::UpdateBegin { app: &old_app.app_name });
    let latest = app.latest_installable_version(log)?;
    if let Some(previous) = &old_app.versions.update_largest_with(&latest) {
      log(Event::UpdateNewVersion {
        old_version: previous,
        new_version: &latest,
      });
    } else {
      log(Event::UpdateAlreadyNewest);
    }
  }
  config.save()?;
  Ok(ExitCode::SUCCESS)
}

#[derive(Debug, PartialEq)]
pub(crate) struct Args {
  pub(crate) verbose: bool,
}
