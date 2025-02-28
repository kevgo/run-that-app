use crate::applications::ApplicationName;
use crate::configuration;
use crate::prelude::*;
use std::process::ExitCode;

pub(crate) fn add(app_name: ApplicationName) -> Result<ExitCode> {
  // determine the latest version of the app
  // create config file if necessary
  // add the app to the config file
  configuration::File::create()?;
  println!("Created file {}", configuration::FILE_NAME);
  Ok(ExitCode::SUCCESS)
}
