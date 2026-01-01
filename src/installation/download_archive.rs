use super::{BinFolder, Outcome};
use crate::applications::{AppDefinition, carrier};
use crate::configuration::Version;
use crate::context::RuntimeContext;
use crate::download::Url;
use crate::error::{Result, UserError};
use crate::{archives, download, filesystem};
use std::path::Path;

/// downloads and unpacks the content of an archive file
pub(crate) fn run(
  app_definition: &dyn AppDefinition,
  app_folder: &Path,
  version: &Version,
  url: &Url,
  bin_folders: &BinFolder,
  optional: bool,
  ctx: &RuntimeContext,
) -> Result<Outcome> {
  let (app_to_install, executable_name, _args) = carrier(app_definition, version, ctx.platform);
  let app_name = app_to_install.name();
  // download the archive file
  let Some(artifact) = download::artifact(url, &app_name, optional, ctx.log)? else {
    return Ok(Outcome::NotInstalled);
  };
  // determine the archive type
  let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
    return Err(UserError::UnknownArchive(artifact.filename));
  };
  // extract the archive
  archive.extract_all(app_folder, ctx.log)?;
  let executable_filename = executable_name.platform_path(ctx.platform.os);
  // verify that all executables that should be there exist and are executable
  for executable_path in bin_folders.executable_paths(app_folder, &executable_filename) {
    filesystem::set_executable_bit(&executable_path);
    // set the executable bit of all executable files that this app provides
    for other_executable in app_definition.additional_executables() {
      let other_executable_filename = other_executable.platform_path(ctx.platform.os);
      for other_executable_path in bin_folders.executable_paths(app_folder, &other_executable_filename) {
        filesystem::set_executable_bit(&other_executable_path);
      }
    }
  }
  Ok(Outcome::Installed)
}
