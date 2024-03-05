use super::{AppName, Versions};

#[derive(Debug, PartialEq)]
pub struct AppVersions {
    pub app: AppName,
    pub versions: Versions,
}
