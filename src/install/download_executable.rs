use std::path::PathBuf;

use crate::apps::App;
use crate::config::Version;
use crate::output::Output;
use crate::platform::Platform;
use crate::yard::Yard;
use crate::{download, filesystem, yard, Result};
use colored::Colorize;

/// defines the information needed for RTA to download a pre-compiled application executable
pub trait DownloadExecutable: App {
    fn artifact_url(&self, version: &Version, platform: Platform) -> String;

    fn executable_location(&self, version: &Version, platform: Platform, yard: Yard) -> PathBuf {
        yard.app_folder(&self.name(), version).join(self.executable_filename(platform))
    }
}

/// downloads an uncompressed precompiled binary
pub fn install(app: &dyn DownloadExecutable, version: &Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    let Some(artifact) = download::artifact(app.artifact_url(version, platform), &app.name(), output)? else {
        return Ok(false);
    };
    let yard = yard::load_or_create(&yard::production_location()?)?;
    let filepath_on_disk = yard.create_app_folder(&app.name(), version)?.join(app.executable_filename(platform));
    filesystem::save_executable(artifact.data, &filepath_on_disk, output)?;
    output.println(&format!("{}", "ok".green()));
    Ok(true)
}
