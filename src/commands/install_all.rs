use crate::applications::Apps;
use crate::commands::run::load_or_install_apps;
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::yard::{self, Yard};
use crate::{configuration, logging, platform};
use std::process::ExitCode;

pub(crate) fn install_all(apps: &Apps) -> Result<ExitCode> {
  let config_file = configuration::File::load(apps)?;
  let log = logging::new(false);
  let platform = platform::detect(log)?;
  let yard = Yard::load_or_create(&yard::production_location()?)?;
  let ctx = RuntimeContext {
    platform,
    yard: &yard,
    config_file: &config_file,
    log,
  };
  let _ = load_or_install_apps(&config_file.apps, apps, true, false, &ctx)?;
  Ok(ExitCode::SUCCESS)
}
