use super::Outcome;
use crate::applications::AppDefinition;
use crate::configuration::Version;
use crate::context::RuntimeContext;
use crate::download::Url;
use crate::error::Result;
use crate::{download, filesystem};
use std::path::Path;

/// downloads an uncompressed precompiled binary
pub fn run(app_definition: &dyn AppDefinition, app_folder: &Path, version: &Version, url: &Url, optional: bool, ctx: &RuntimeContext) -> Result<Outcome> {
  let Some(artifact) = download::artifact(url, &app_definition.name(), version, optional, ctx.log)? else {
    return Ok(Outcome::NotInstalled { app: app_definition.name() });
  };
  let filepath_on_disk = app_folder.join(app_definition.executable_filename().platform_path(ctx.platform.os).as_ref());
  filesystem::save_executable(artifact.data, &filepath_on_disk, ctx.log);
  Ok(Outcome::Installed)
}
