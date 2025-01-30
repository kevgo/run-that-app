use crate::configuration::{File, FILE_NAME};
use crate::prelude::*;
use std::process::ExitCode;

pub(crate) fn setup() -> Result<ExitCode> {
  File::create()?;
  println!("Created file {FILE_NAME}");
  Ok(ExitCode::SUCCESS)
}
