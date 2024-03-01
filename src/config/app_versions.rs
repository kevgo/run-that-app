use super::{AppVersion, Version};

#[derive(Debug, Default, PartialEq)]
pub struct AppVersions {
    pub name: String,
    pub versions: Vec<Version>,
}

impl From<AppVersion> for AppVersions {
    fn from(app_version: AppVersion) -> Self {
        AppVersions {
            name: app_version.name,
            versions: vec![app_version.version],
        }
    }
}
