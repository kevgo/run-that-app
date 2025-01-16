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
  load_or_install(
    app_to_install,
    &RequestedVersion::Yard(version.to_owned()),
    platform,
    optional,
    yard,
    config_file,
    log,
  )?;
  if let Some(executable) = load(app_to_install, version, yard, executable_filename) {
    Ok(Outcome::Installed { executable })
  } else {
    Err(UserError::ExecutableNotFoundAfterInstallation {
      app: app_to_install.name().to_string(),
      executable_path: executable_filename.to_string(),
    })
  }
}

pub fn load(app_to_install: &dyn App, version: &Version, yard: &Yard, executable_filename: &str) -> Option<Executable> {
  let executable_path = yard.app_folder(&app_to_install.name(), version).join(executable_filename);
  if executable_path.exists() {
    Some(Executable(executable_path))
  } else {
    None
  }
}
