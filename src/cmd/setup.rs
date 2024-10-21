use crate::config::{File, FILE_NAME};
use crate::prelude::*;
use std::process::ExitCode;

pub fn setup() -> Result<ExitCode> {
  File::create()?;
  println!("Created file {FILE_NAME}");
  Ok(ExitCode::SUCCESS)
}
