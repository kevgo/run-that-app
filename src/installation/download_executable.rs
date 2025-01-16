use super::Outcome;
use crate::applications::App;
use crate::configuration::Version;
use crate::logging::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{download, filesystem};

/// defines the information needed to download a pre-compiled application executable
pub trait DownloadExecutable: App {
  /// the URL at which to download the executable
  fn download_url(&self, version: &Version, platform: Platform) -> String;
}

/// downloads an uncompressed precompiled binary
pub fn install(app: &dyn App, url: &str, version: &Version, platform: Platform, optional: bool, yard: &Yard, log: Log) -> Result<Outcome> {
  let Some(artifact) = download::artifact(url, &app.name(), optional, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let filepath_on_disk = yard.create_app_folder(&app.name(), version)?.join(app.executable_filename(platform));
  let executable = filesystem::save_executable(artifact.data, &filepath_on_disk, log)?;
  Ok(Outcome::Installed { executable })
}

pub fn load(app: &dyn App, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
  let app_folder = yard.app_folder(&app.name(), version);
  let executable_path_absolute = app_folder.join(app.executable_filename(platform));
  if executable_path_absolute.exists() {
    return Some(Executable(executable_path_absolute));
  }
  None
}
