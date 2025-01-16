use super::Outcome;
use crate::applications::App;
use crate::configuration::Version;
use crate::logging::Log;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{archives, download};
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// downloads and unpacks the content of an archive file
pub fn run(app: &dyn App, version: &Version, url: &str, executable_path_in_archive: &str, optional: bool, yard: &Yard, log: Log) -> Result<Outcome> {
  let Some(artifact) = download::artifact(url, &app.name(), optional, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let app_folder = yard.create_app_folder(&app.name(), version)?;
  let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
    return Err(UserError::UnknownArchive(artifact.filename));
  };
  archive.extract_all(&app_folder, log)?;
  let Some(executable) = load(app, version, executable_path_in_archive, yard) else {
    return Err(UserError::ExecutableNotFoundAfterInstallation {
      app: app.name().to_string(),
      executable_path: executable_path_in_archive.to_string(),
    });
  };
  #[cfg(unix)]
  make_executable_unix(&executable.0)?;
  #[cfg(windows)]
  make_executable_windows(&executable.0);
  Ok(Outcome::Installed { executable })
}

/// tries to load the executable of the given app, if it was installed by downloading
pub fn load(app: &dyn App, version: &Version, executable_path_in_archive: &str, yard: &Yard) -> Option<Executable> {
  let app_folder = yard.app_folder(&app.name(), version);
  let executable_path_absolute = app_folder.join(executable_path_in_archive);
  if executable_path_absolute.exists() {
    return Some(Executable(executable_path_absolute));
  }
  None
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
