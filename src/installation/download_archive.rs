use super::{BinFolder, Outcome};
use crate::applications::AppDefinition;
use crate::configuration::Version;
use crate::logging::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::yard::Yard;
use crate::{archives, download};
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// downloads and unpacks the content of an archive file
pub fn run(
  app_definition: &dyn AppDefinition,
  version: &Version,
  url: &str,
  bin_folders: &BinFolder,
  optional: bool,
  platform: Platform,
  yard: &Yard,
  log: Log,
) -> Result<Outcome> {
  let Some(artifact) = download::artifact(url, &app_definition.name(), optional, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let app_folder = yard.create_app_folder(&app_definition.name(), version)?;
  let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
    return Err(UserError::UnknownArchive(artifact.filename));
  };
  // extract the archive
  archive.extract_all(&app_folder, log)?;
  let executable_filename = app_definition.default_executable_filename().platform_path(platform.os);
  // verify that all executables that should be there exist and are executable
  for bin_folder in bin_folders.executable_paths(&app_folder, &executable_filename) {
    let bin_path = app_folder.join(bin_folder);
    make_executable(&bin_path.join(app_definition.default_executable_filename().platform_path(platform.os)));
    // set the executable bit of all executable files that this app provides
    for other_executable in app_definition.additional_executables() {
      make_executable(&bin_path.join(other_executable.platform_path(platform.os)));
    }
  }
  Ok(Outcome::Installed)
}

fn make_executable(filepath: &Path) {
  #[cfg(unix)]
  let _ = make_executable_unix(filepath);
  #[cfg(windows)]
  make_executable_windows(filepath);
}

#[cfg(windows)]
fn make_executable_windows(_filepath: &Path) {
  // Windows does not have file permissions --> nothing to do here
}

#[cfg(unix)]
fn make_executable_unix(filepath: &Path) -> Result<()> {
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
