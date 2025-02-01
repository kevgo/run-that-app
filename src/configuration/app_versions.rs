use super::RequestedVersions;
use crate::applications::ApplicationName;

#[derive(Debug, PartialEq)]
pub(crate) struct AppVersions {
  pub(crate) app_name: ApplicationName,
  pub(crate) versions: RequestedVersions,
}
