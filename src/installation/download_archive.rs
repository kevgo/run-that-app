use super::Outcome;
use crate::applications::App;
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

/// defines the information needed to download and extract an archive containing an app
pub trait DownloadArchive: App {
  /// the URL of the archive to download
  fn archive_url(&self, version: &Version, platform: Platform) -> String;

  /// the location of the executable within the archive
  fn executable_path_in_archive(&self, version: &Version, platform: Platform) -> String;
}

/// downloads and unpacks the content of an archive file
pub fn run(app: &dyn DownloadArchive, version: &Version, platform: Platform, optional: bool, yard: &Yard, log: Log) -> Result<Outcome> {
  let Some(artifact) = download::artifact(app.archive_url(version, platform), &app.name(), optional, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let app_folder = yard.create_app_folder(&app.name(), version)?;
  let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
    return Err(UserError::UnknownArchive(artifact.filename));
  };
  archive.extract_all(&app_folder, log)?;
  let executable_path_relative = app.executable_path_in_archive(version, platform);
  let executable_path_absolute = app_folder.join(executable_path_relative);
  #[cfg(unix)]
  make_executable_unix(&executable_path_absolute)?;
  #[cfg(windows)]
  make_executable_windows(&executable_path_absolute);
  Ok(Outcome::Installed)
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
