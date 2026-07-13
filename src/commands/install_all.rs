use crate::applications::{ApplicationName, Apps};
use crate::context::RuntimeContext;
use crate::error::Result;
use crate::executables::load_or_install_apps;
use crate::yard::{self, Yard};
use crate::{configuration, logging, platform};
use std::process::ExitCode;

pub fn install_all(apps: &Apps) -> Result<ExitCode> {
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
  let app_names_to_install: Vec<&ApplicationName> = config_file.apps.iter().map(|app| &app.app_name).collect();
  let apps_to_install = apps.lookup_many(&app_names_to_install)?;
  let _ = load_or_install_apps(apps_to_install, apps, true, &ctx)?;
  Ok(ExitCode::SUCCESS)
}
