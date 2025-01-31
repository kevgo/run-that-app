use super::{ApplicationName, RequestedVersions};

#[derive(Debug, PartialEq)]
pub(crate) struct AppVersions<'a> {
  pub(crate) app_name: ApplicationName<'a>,
  pub(crate) versions: RequestedVersions,
}
