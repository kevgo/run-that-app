use super::Outcome;
use crate::applications::AppDefinition;
use crate::configuration::Version;
use crate::logging::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::yard::Yard;
use crate::{download, filesystem};

/// downloads an uncompressed precompiled binary
pub fn run(app: &dyn AppDefinition, url: &str, version: &Version, platform: Platform, optional: bool, yard: &Yard, log: Log) -> Result<Outcome> {
  let Some(artifact) = download::artifact(url, &app.name(), optional, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let filepath_on_disk = yard
    .create_app_folder(&app.name(), version)?
    .join(app.default_executable_filename().platform_path(platform.os));
  filesystem::save_executable(artifact.data, &filepath_on_disk, log)?;
  Ok(Outcome::Installed)
}
