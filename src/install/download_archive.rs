use colored::Colorize;

use crate::apps::App;
use crate::config::Version;
use crate::output::Output;
use crate::platform::Platform;
use crate::UserError;
use crate::{archives, yard};
use crate::{download, Result};
use std::fs;

pub trait Data: App {
    /// provides the URL of the archive to download
    fn archive_url(&self, version: &Version, platform: Platform) -> String;
}

/// downloads and unpacks the content of an archive file
pub fn run(app: &dyn Data, version: &Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    let Some(artifact) = download::artifact(app.archive_url(version, platform), &app.name(), output)? else {
        return Ok(false);
    };
    let yard = yard::load_or_create(&yard::production_location()?)?;
    let app_folder = yard.app_folder(&app.name(), version);
    fs::create_dir_all(&app_folder).map_err(|err| UserError::CannotCreateFolder {
        folder: app_folder.clone(),
        reason: err.to_string(),
    })?;
    let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
        return Err(UserError::UnknownArchive(artifact.filename));
    };
    archive.extract_all(&app_folder, output)?;
    output.println("ok".green().bold().as_ref());
    Ok(true)
}
