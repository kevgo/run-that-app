use super::Outcome;
use crate::applications::AppDefinition;
use crate::context::RuntimeContext;
use crate::download::URL;
use crate::error::Result;
use crate::{download, filesystem};
use std::path::Path;

/// downloads an uncompressed precompiled binary
pub(crate) fn run(app_definition: &dyn AppDefinition, app_folder: &Path, url: &URL, optional: bool, ctx: &RuntimeContext) -> Result<Outcome> {
  let Some(artifact) = download::artifact(url, &app_definition.name(), optional, ctx.log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let filepath_on_disk = app_folder.join(app_definition.executable_filename().platform_path(ctx.platform.os));
  filesystem::save_executable(artifact.data, &filepath_on_disk, ctx.log)?;
  Ok(Outcome::Installed)
}
