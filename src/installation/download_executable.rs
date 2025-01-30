use super::Outcome;
use crate::applications::AppDefinition;
use crate::configuration::Version;
use crate::logging::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::yard::Yard;
use crate::{download, filesystem};

/// downloads an uncompressed precompiled binary
pub(crate) fn run(
  app_definition: &dyn AppDefinition,
  url: &str,
  version: &Version,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  log: Log,
) -> Result<Outcome> {
  let app_name = app_definition.app_name();
  let Some(artifact) = download::artifact(url, &app_name, optional, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let filepath_on_disk = yard
    .create_app_folder(&app_name, version)?
    .join(app_definition.default_executable_filename().platform_path(platform.os));
  filesystem::save_executable(artifact.data, &filepath_on_disk, log)?;
  Ok(Outcome::Installed)
}
