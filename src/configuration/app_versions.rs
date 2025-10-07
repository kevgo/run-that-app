use super::RequestedVersions;
use crate::applications::ApplicationName;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub(crate) struct AppVersions {
  pub(crate) app_name: ApplicationName,
  pub(crate) versions: RequestedVersions,
}

impl Eq for AppVersions {}

impl PartialOrd for AppVersions {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for AppVersions {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    match self.app_name.as_str().cmp(other.app_name.as_str()) {
      Ordering::Equal => self.versions.cmp(&other.versions),
      other => other,
    }
  }
}
