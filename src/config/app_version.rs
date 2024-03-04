use super::{AppName, Version};

/// an entry in the configuration file
#[derive(Debug, PartialEq)]
pub struct AppVersion {
    pub app: AppName,
    pub version: Option<Version>,
}
