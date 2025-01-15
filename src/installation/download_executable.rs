use super::Outcome;
use crate::applications::App;
use crate::configuration::Version;
use crate::logging::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::yard::Yard;
use crate::{download, filesystem};

/// defines the information needed to download a pre-compiled application executable
pub trait DownloadExecutable: App {
  /// the URL at which to download the executable
  fn download_url(&self, version: &Version, platform: Platform) -> String;
}

/// downloads an uncompressed precompiled binary
pub fn install(app: &dyn DownloadExecutable, version: &Version, platform: Platform, optional: bool, yard: &Yard, log: Log) -> Result<Outcome> {
  let url = app.download_url(version, platform);
  let Some(artifact) = download::artifact(url, &app.name(), optional, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let filepath_on_disk = yard.create_app_folder(&app.name(), version)?.join(app.executable_filename(platform));
  filesystem::save_executable(artifact.data, &filepath_on_disk, log)?;
  Ok(Outcome::Installed)
}
