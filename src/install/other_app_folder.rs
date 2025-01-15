use super::Outcome;
use crate::apps::App;
use crate::cmd::run::load_or_install;
use crate::config::{RequestedVersion, Version};
use crate::logger::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::CallSignature;
use crate::yard::Yard;
use crate::{apps, config};

/// defines the information needed for apps whose executable is shipped as part of another app
pub trait ViaAnotherApp: App {
  /// the application that ships the executable of this app
  fn app_to_install(&self) -> Box<dyn App>;

  /// location of this app's executable within the archive of the other app
  fn call_signature_for_other_app(&self, platform: Platform) -> CallSignature;
}

pub fn install_other_app(
  app: &dyn ViaAnotherApp,
  version: &Version,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  config_file: &config::File,
  log: Log,
) -> Result<Outcome> {
  let app_to_install = app.app_to_install();
  let all_apps = apps::all();
  let app = all_apps.lookup(&app_to_install.name())?;
  // Note: we know it must be the Yard variant here.
  // At this point we are installing the app.
  // Only Yard variants get installed. The Path variant doesn't get installed.
  load_or_install(app, &RequestedVersion::Yard(version.to_owned()), platform, optional, yard, config_file, log)?;
  Ok(Outcome::Installed)
}
