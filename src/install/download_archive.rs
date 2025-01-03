use super::Outcome;
use crate::apps::App;
use crate::config::Version;
use crate::logger::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::yard::Yard;
use crate::{archives, download};

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
  Ok(Outcome::Installed)
}
