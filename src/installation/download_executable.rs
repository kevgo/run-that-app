use super::Outcome;
use crate::applications::AppDefinition;
use crate::logging::Log;
use crate::platform::Platform;
use crate::error::Result;
use crate::{download, filesystem};
use std::path::Path;

/// downloads an uncompressed precompiled binary
pub(crate) fn run(app_definition: &dyn AppDefinition, app_folder: &Path, url: &str, platform: Platform, optional: bool, log: Log) -> Result<Outcome> {
  let Some(artifact) = download::artifact(url, &app_definition.app_name(), optional, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let filepath_on_disk = app_folder.join(app_definition.executable_filename().platform_path(platform.os));
  filesystem::save_executable(artifact.data, &filepath_on_disk, log)?;
  Ok(Outcome::Installed)
}
