use crate::configuration;
use crate::prelude::*;
use std::process::ExitCode;

pub(crate) fn setup() -> Result<ExitCode> {
  configuration::File::create()?;
  println!("Created file {}", configuration::FILE_NAME);
  Ok(ExitCode::SUCCESS)
}
