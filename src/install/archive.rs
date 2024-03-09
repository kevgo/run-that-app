use crate::apps::App;
use crate::archives;
use crate::config::{AppName, Version};
use crate::output::Output;
use crate::platform::Platform;
use crate::UserError;
use crate::{download, Result};
use std::fs;
use std::path::Path;

pub trait InstallByArchive: App {
    /// provides the URL of the archive to download
    fn archive_url(&self, version: &Version, platform: Platform) -> String;
}

/// downloads and unpacks the content of an archive file
pub fn install(app: &dyn InstallByArchive, version: &Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    let Some(artifact) = download::artifact(app.archive_url(platform), &app.name(), output)? else {
        return Ok(false);
    };
    fs::create_dir_all(&app.dir_on_disk).map_err(|err| UserError::CannotCreateFolder {
        folder: args.dir_on_disk.to_path_buf(),
        reason: err.to_string(),
    })?;
    let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
        return Err(UserError::UnknownArchive(artifact.filename));
    };
    archive.extract_all(&args.dir_on_disk, args.output)?;
    Ok(true)
}

pub struct InstallArgs<'a> {
    pub app_name: &'a AppName,
    pub artifact_url: String,
    pub dir_on_disk: &'a Path,
    pub executable_locations: Vec<String>,
    pub output: &'a dyn Output,
}
