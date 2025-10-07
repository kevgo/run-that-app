use super::RequestedVersions;
use crate::applications::ApplicationName;

#[derive(Debug, PartialEq)]
pub(crate) struct AppVersions {
  pub(crate) app_name: ApplicationName,
  pub(crate) versions: RequestedVersions,
}

impl Ord for AppVersions {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    // TODO: sort first by app name, then by version
  }
}
