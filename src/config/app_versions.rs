use super::{AppName, RequestedVersions};

#[derive(Debug, PartialEq)]
pub struct AppVersions {
  pub app_name: AppName,
  pub versions: RequestedVersions,
}
