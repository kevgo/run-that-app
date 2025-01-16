use super::Outcome;
use crate::applications::App;
use crate::commands::run::load_or_install;
use crate::configuration;
use crate::configuration::{RequestedVersion, Version};
use crate::logging::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::yard::Yard;
use std::path::PathBuf;

pub fn install_other_app(
  app_to_install: &dyn App,
  version: &Version,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  executable_filename: &str,
  config_file: &configuration::File,
  log: Log,
) -> Result<Outcome> {
  // Note: we know it must be the Yard variant here.
  // At this point we are installing the app.
  // Only Yard variants get installed. The Path variant doesn't get installed.
  load_or_install(app_to_install, &RequestedVersion::Yard(version.to_owned()), platform, optional, yard, config_file, log)?;
  let executable_path = executable_path(app_to_install, version, yard, executable_filename);
  if !executable_path.exists() {
    return Err(UserError::InternalError {
      desc: format!("executable not found after installing via other app {}: {}", app_to_install.name(), executable_path.to_string_lossy()),
    });
  }
  Ok(Outcome::Installed {
    executable: Executable(executable_path),
  })
}

pub fn executable_path(app_to_install: &dyn App, version: &Version, yard: &Yard, executable_path_in_other_yard: &str) -> PathBuf {
  yard.app_folder(&app_to_install.name(), version).join(executable_path_in_other_yard)
}
