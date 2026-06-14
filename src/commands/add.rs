use crate::applications::{ApplicationName, Apps};
use crate::error::Result;
use crate::{configuration, logging};
use std::process::ExitCode;

pub fn add(args: AddArgs, apps: &Apps) -> Result<ExitCode> {
  let log = logging::new(args.verbose);
  let app = apps.lookup(args.app_name)?;
  let version = app.latest_installable_version(log)?;
  if let Some(config_file) = configuration::File::read(apps)? {
    config_file.add(app.name(), version.clone())?;
  } else {
    configuration::File::create(&app.name(), &version.clone())?;
  }
  eprintln!("added {}@{} to {}", app.name(), &version, configuration::FILE_NAME);
  Ok(ExitCode::SUCCESS)
}

/// named arguments for the [`add`] command
#[derive(Debug, PartialEq)]
pub struct AddArgs {
  pub app_name: ApplicationName,
  pub verbose: bool,
}
