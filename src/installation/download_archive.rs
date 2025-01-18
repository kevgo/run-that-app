use super::Outcome;
use crate::applications::App;
use crate::configuration::Version;
use crate::logging::Log;
use crate::prelude::*;
use crate::yard::Yard;
use crate::{archives, download};
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// downloads and unpacks the content of an archive file
pub fn run(app: &dyn App, version: &Version, url: &str, bin_folders: &[&str], optional: bool, yard: &Yard, log: Log) -> Result<Outcome> {
  let Some(artifact) = download::artifact(url, &app.name(), optional, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let app_folder = yard.create_app_folder(&app.name(), version)?;
  let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
    return Err(UserError::UnknownArchive(artifact.filename));
  };
  // extract the archive
  archive.extract_all(&app_folder, log)?;
  // verify that all executables that should be there exist and are executable
  for bin_folder in bin_folders {
    let bin_path = app_folder.join(bin_folder);
    let default_executable_path = bin_path.join(app.executable_filename(platform));
    if !executable_path.exists() {
      return Err(UserError::InternalError {
        desc: format!("executable not found after downloading archive: {}", executable_path.to_string_lossy()),
      });
    };
    // set the executable bit of all executable files that this app provides
    #[cfg(unix)]
    make_executable_unix(&executable_path)?;
    for other_executable in app.other_executables() {
      // TODO: determine the full path to the executable here
      make_executable_unix(&other_executable)?;
    }
    #[cfg(windows)]
    make_executable_windows(&executable_path);
  }
  Ok(Outcome::Installed)
}

/// tries to load the executable of the given app, if it was installed by downloading
pub fn executable_path(app: &dyn App, version: &Version, executable_path_in_archive: &str, yard: &Yard) -> PathBuf {
  yard.app_folder(&app.name(), version).join(executable_path_in_archive)
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
