use crate::applications::Apps;
use crate::commands::install;
use crate::error::Result;
use crate::yard::{self, Yard};
use std::process::ExitCode;

pub fn reinstall(args: install::InstallArgs, apps: &Apps) -> Result<ExitCode> {
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  yard.delete_app_folders(&args.app_name, args.version.as_ref())?;
  install(args, apps)
}
