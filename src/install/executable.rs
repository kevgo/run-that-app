use crate::apps::App;
use crate::config::{AppName, Version};
use crate::output::Output;
use crate::platform::Platform;
use crate::{download, filesystem, yard, Result};
use colored::Colorize;
use std::path::Path;

pub trait DownloadExecutable: App {
    fn artifact_url(&self, version: &Version, platform: Platform) -> String;
}

/// downloads an uncompressed precompiled binary
pub fn install(app: &dyn DownloadExecutable, version: &Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    let url = app.artifact_url(version, platform);
    let Some(artifact) = download::artifact(app.artifact_url(version, platform), &app.name(), output)? else {
        return Ok(false);
    };
    let yard = yard::load_or_create(&yard::production_location()?)?;
    let filepath_on_disk = yard.app_folder(&app.yard_app(), version).join(app.executable_filename(platform));
    filesystem::create_parent(&filepath_on_disk)?;
    output.println(&format!("{}", "ok".green()));
    Ok(true)
}

pub struct InstallArgs<'a> {
    pub app_name: &'a AppName,
    pub artifact_url: String,
    pub filepath_on_disk: &'a Path,
    pub output: &'a dyn Output,
}
