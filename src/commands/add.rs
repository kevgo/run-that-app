use crate::applications::ApplicationName;
use crate::applications::Apps;
use crate::configuration;
use crate::logging;
use crate::prelude::*;
use std::process::ExitCode;

pub(crate) fn add(app_name: ApplicationName, apps: &Apps) -> Result<ExitCode> {
  let log = logging::new(args.verbose);
  // determine the latest version of the app
  let app = apps.lookup(app_name)?;
  let version = app.latest_installable_version(log)

  // create config file if necessary
  // add the app to the config file
  configuration::File::create()?;
  println!("Created file {}", configuration::FILE_NAME);
  Ok(ExitCode::SUCCESS)
}
