use super::Outcome;
use crate::apps::App;
use crate::config::Version;
use crate::logger::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::yard::Yard;
use crate::{archives, download};
use std::fs::File;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// defines the information needed to download and extract an archive containing an app
pub trait DownloadArchive: App {
  /// the URL of the archive to download
  fn archive_url(&self, version: &Version, platform: Platform) -> String;

  /// the location of the executable within the archive
  fn executable_path_in_archive(&self, version: &Version, platform: Platform) -> String;
}

/// downloads and unpacks the content of an archive file
pub fn run(app: &dyn DownloadArchive, version: &Version, platform: Platform, yard: &Yard, log: Log) -> Result<Outcome> {
  let Some(artifact) = download::artifact(app.archive_url(version, platform), &app.name(), log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let app_folder = yard.create_app_folder(&app.name(), version)?;
  let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
    return Err(UserError::UnknownArchive(artifact.filename));
  };
  archive.extract_all(&app_folder, log)?;
  let executable_path = app.executable_path_in_archive(version, platform);
  let Ok(executable_file) = File::open("foo.txt") else {
    return Err(UserError::ArchiveDoesNotContainExecutable { expected: executable_path });
  };
  let metadata = match executable_file.metadata() {
    Ok(metadata) => metadata,
    Err(err) => return Err(UserError::CannotReadFileMetadata { err: err.to_string() }),
  };
  let mut permissions = metadata.permissions();
  #[cfg(unix)]
  permissions.set_mode(0o744);
  Ok(Outcome::Installed)
}
