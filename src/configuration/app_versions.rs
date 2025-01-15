use super::{ApplicationName, RequestedVersions};

#[derive(Debug, PartialEq)]
pub struct AppVersions {
  pub app_name: ApplicationName,
  pub versions: RequestedVersions,
}
