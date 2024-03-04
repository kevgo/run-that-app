use super::{AppName, AppVersion, Version};

#[derive(Debug, PartialEq)]
pub struct AppVersions {
    pub app: AppName,
    pub versions: Vec<Version>,
}

impl From<AppVersion> for AppVersions {
    fn from(app_version: AppVersion) -> Self {
        AppVersions {
            app: app_version.app,
            versions: vec![app_version.version],
        }
    }
}
