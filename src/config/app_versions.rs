use super::{AppName, Versions};
use crate::cli::AppVersion;

#[derive(Debug, PartialEq)]
pub struct AppVersions {
    pub app: AppName,
    pub versions: Versions,
}

impl From<AppVersion> for AppVersions {
    fn from(app_version: AppVersion) -> Self {
        AppVersions {
            app: app_version.app,
            versions: Versions::from(app_version.version),
        }
    }
}
