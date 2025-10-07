use super::{BinFolder, Outcome};
use crate::applications::{AppDefinition, carrier};
use crate::configuration::Version;
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::logging::Log;
use crate::{archives, download};
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// downloads and unpacks the content of an archive file
pub(crate) fn run(
  app_definition: &dyn AppDefinition,
  app_folder: &Path,
  version: &Version,
  url: &str,
  bin_folders: &BinFolder,
  optional: bool,
  ctx: &RuntimeContext,
) -> Result<Outcome> {
  let (app_to_install, executable_name, _args) = carrier(app_definition, version, ctx.platform);
  let app_name = app_to_install.app_name();
  let Some(artifact) = download::artifact(url, &app_name, optional, ctx.log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
    return Err(UserError::UnknownArchive(artifact.filename));
  };
  // extract the archive
  archive.extract_all(app_folder, ctx.log)?;
  let executable_filename = executable_name.platform_path(ctx.platform.os);
  // verify that all executables that should be there exist and are executable
  for executable_path in bin_folders.executable_paths(app_folder, &executable_filename) {
    make_executable(&executable_path, ctx.log);
    // set the executable bit of all executable files that this app provides
    for other_executable in app_definition.additional_executables() {
      let other_executable_filename = other_executable.platform_path(ctx.platform.os);
      for other_executable_path in bin_folders.executable_paths(app_folder, &other_executable_filename) {
        make_executable(&other_executable_path, ctx.log);
      }
    }
  }
  Ok(Outcome::Installed)
}

fn make_executable(filepath: &Path, log: Log) {
  #[cfg(unix)]
  let _ = make_executable_unix(filepath, log);
  #[cfg(windows)]
  make_executable_windows(filepath, log);
}

#[cfg(windows)]
fn make_executable_windows(_filepath: &Path, _log: Log) {
  // Windows does not have file permissions --> nothing to do here
}

#[cfg(unix)]
fn make_executable_unix(filepath: &Path, log: Log) -> Result<()> {
  use crate::logging::Event;

  log(Event::MakeExecutable { file: filepath });
  let Ok(executable_file) = fs::File::open(filepath) else {
    return Err(UserError::ArchiveDoesNotContainExecutable {
      expected: filepath.to_string_lossy().to_string(),
    });
  };
  let metadata = match executable_file.metadata() {
    Ok(metadata) => metadata,
    Err(err) => {
      return Err(UserError::CannotReadFileMetadata { err: err.to_string() });
    }
  };
  let mut permissions = metadata.permissions();
  if permissions.mode() & 0o100 == 0 {
    permissions.set_mode(0o744);
    if let Err(err) = fs::set_permissions(filepath, permissions) {
      return Err(UserError::CannotSetFilePermissions {
        path: filepath.to_string_lossy().to_string(),
        err: err.to_string(),
      });
    }
  }
  Ok(())
}
