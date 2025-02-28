use crate::applications;
use crate::applications::ApplicationName;
use crate::configuration;
use crate::logging;
use crate::prelude::*;
use std::process::ExitCode;

pub(crate) fn add(args: Args) -> Result<ExitCode> {
  let apps = applications::all();
  let log = logging::new(args.verbose);
  // determine the latest version of the app
  let app = apps.lookup(args.app_name)?;
  let version = app.latest_installable_version(log)?;
  // create config file if necessary
  if let Some(config_file) = configuration::File::read(&apps)? {
    config_file.add(app, version)?;
  } else {
    configuration::File::create(app.app_name(), version)?;
  }
  println!("added {app}@{version} to {}", configuration::FILE_NAME);
  Ok(ExitCode::SUCCESS)
}

#[derive(Debug, PartialEq)]
pub(crate) struct Args {
  pub(crate) app_name: ApplicationName,
  pub(crate) verbose: bool,
}
