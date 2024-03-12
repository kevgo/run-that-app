use crate::apps::App;
use crate::config::Version;
use crate::output::Output;
use crate::platform::Platform;
use crate::UserError;
use crate::{archives, yard};
use crate::{download, Result};
use colored::Colorize;

/// defines the information needed to download and extract an archive containing an app
pub trait DownloadArchive: App {
    /// provides the URL of the archive to download
    fn archive_url(&self, version: &Version, platform: Platform) -> String;

    /// the location of the executable within the archive
    fn executable_path_in_archive(&self, version: &Version, platform: Platform) -> String;
}

/// downloads and unpacks the content of an archive file
pub fn run(app: &dyn DownloadArchive, version: &Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    let Some(artifact) = download::artifact(app.archive_url(version, platform), &app.name(), output)? else {
        return Ok(false);
    };
    let yard = yard::load_or_create(&yard::production_location()?)?;
    let app_folder = yard.create_app_folder(&app.name(), version)?;
    let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
        return Err(UserError::UnknownArchive(artifact.filename));
    };
    archive.extract_all(&app_folder, output)?;
    output.println("ok".green().bold().as_ref());
    Ok(true)
}
