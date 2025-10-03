use crate::applications::{ApplicationName, Apps};
use crate::prelude::*;
use crate::{configuration, logging};
use std::process::ExitCode;

pub(crate) fn add(args: Args, apps: &Apps) -> Result<ExitCode> {
  let log = logging::new(args.verbose);
  let app = apps.lookup(args.app_name)?.clone();
  let version = app.latest_installable_version(log)?;
  if let Some(config_file) = configuration::File::read(apps)? {
    config_file.add(app.app_name(), version.clone())?;
  } else {
    configuration::File::create(&app.app_name(), &version.clone())?;
  }
  println!("added {}@{} to {}", &app, &version, configuration::FILE_NAME);
  Ok(ExitCode::SUCCESS)
}

#[derive(Debug, PartialEq)]
pub(crate) struct Args {
  pub(crate) app_name: ApplicationName,
  pub(crate) verbose: bool,
}
