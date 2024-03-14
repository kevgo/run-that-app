use crate::apps::App;
use crate::config::Version;
use crate::logger::Log;
use crate::platform::Platform;
use crate::yard::Yard;
use crate::{download, filesystem, Result};

/// defines the information needed to download a pre-compiled application executable
pub trait DownloadExecutable: App {
    /// the URL at which to download the executable
    fn download_url(&self, version: &Version, platform: Platform) -> String;
}

/// downloads an uncompressed precompiled binary
pub fn install(app: &dyn DownloadExecutable, version: &Version, platform: Platform, yard: &Yard, log: Log) -> Result<bool> {
    let url = app.download_url(version, platform);
    let Some(artifact) = download::artifact(url, &app.name(), log)? else {
        return Ok(false);
    };
    let filepath_on_disk = yard.create_app_folder(&app.name(), version)?.join(app.executable_filename(platform));
    filesystem::save_executable(artifact.data, &filepath_on_disk, log)?;
    Ok(true)
}
