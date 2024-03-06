use super::{AppName, RequestedVersions};

#[derive(Debug, PartialEq)]
pub struct AppVersions {
    pub app: AppName,
    pub versions: RequestedVersions,
}
