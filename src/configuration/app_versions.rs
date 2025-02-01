use super::{ApplicationName, RequestedVersions};

#[derive(Debug, PartialEq)]
pub(crate) struct AppVersions {
  pub(crate) app_name: ApplicationName,
  pub(crate) versions: RequestedVersions,
}
