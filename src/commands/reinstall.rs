use crate::applications::Apps;
use crate::commands::install;
use crate::error::Result;
use crate::yard::{self, Yard};
use std::process::ExitCode;

pub(crate) fn reinstall(args: install::Args, apps: &Apps) -> Result<ExitCode> {
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  let _ = yard.delete_app_folder(&args.app_name);
  install(args, apps)
}
