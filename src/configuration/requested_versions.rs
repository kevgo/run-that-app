use super::{File, RequestedVersion, Version};
use crate::applications::ApplicationName;
use crate::prelude::*;

/// a collection of Version instances
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RequestedVersions(pub Vec<RequestedVersion>);

impl RequestedVersions {
  /// Provides the version to use: if the user provided a version to use via CLI, use it.
  /// Otherwise provide the versions from the config file.
  pub(crate) fn determine(app: &ApplicationName, cli_version: Option<&Version>, config_file: &File) -> Result<RequestedVersions> {
    if let Some(version) = cli_version {
      return Ok(RequestedVersions::from(version));
    }
    match config_file.lookup(app) {
      Some(versions) => Ok(RequestedVersions(versions.0.clone())),
      None => Err(UserError::RunRequestMissingVersion),
    }
  }

  /// provides the largest yard version contained in this collection
  fn largest_yard(&self) -> Option<&Version> {
    let mut result = None;
    for version in &self.0 {
      let RequestedVersion::Yard(version) = version else {
        continue;
      };
      match result {
        Some(max) if version > max => result = Some(version),
        Some(_) => {}
        None => result = Some(version),
      }
    }
    result
  }

  #[cfg(test)]
  pub(crate) fn new() -> RequestedVersions {
    RequestedVersions(vec![])
  }

  pub(crate) fn push(&mut self, value: RequestedVersion) {
    self.0.push(value);
  }

  /// Updates the largest non-system version in this collection with the given value.
  /// Returns the value that was replaced.
  pub(crate) fn update_largest_with(&mut self, value: &Version) -> Option<Version> {
    let largest = self.largest_yard()?;
    if largest == value {
      return None;
    }
    let largest = largest.clone();
    let mut updated = None;
    for i in 0..self.0.len() {
      let RequestedVersion::Yard(element) = self.0[i].clone() else {
        continue;
      };
      if element == largest {
        updated = Some(element);
        self.0[i] = RequestedVersion::Yard(value.clone());
      }
    }
    updated
  }
}

impl IntoIterator for RequestedVersions {
  type Item = RequestedVersion;
  type IntoIter = std::vec::IntoIter<RequestedVersion>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a RequestedVersions {
  type Item = &'a RequestedVersion;
  type IntoIter = std::slice::Iter<'a, RequestedVersion>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl From<RequestedVersion> for RequestedVersions {
  fn from(requested_version: RequestedVersion) -> Self {
    RequestedVersions(vec![requested_version])
  }
}

impl From<&RequestedVersions> for RequestedVersions {
  fn from(value: &RequestedVersions) -> Self {
    RequestedVersions(value.0.iter().map(std::borrow::ToOwned::to_owned).collect())
  }
}

impl From<Vec<RequestedVersion>> for RequestedVersions {
  fn from(value: Vec<RequestedVersion>) -> Self {
    RequestedVersions(value)
  }
}

impl From<Vec<Version>> for RequestedVersions {
  fn from(value: Vec<Version>) -> Self {
    let versions: Vec<RequestedVersion> = value.into_iter().map(RequestedVersion::from).collect();
    RequestedVersions::from(versions)
  }
}

impl From<Version> for RequestedVersions {
  fn from(version: Version) -> Self {
    RequestedVersions(vec![RequestedVersion::from(version)])
  }
}

impl From<&Version> for RequestedVersions {
  fn from(version: &Version) -> Self {
    RequestedVersions(vec![RequestedVersion::from(version)])
  }
}

#[cfg(test)]
mod tests {

  mod largest_non_system {
    use crate::configuration::{RequestedVersion, RequestedVersions, Version};

    #[test]
    fn system_and_versions() {
      let versions = RequestedVersions::from(vec![
        RequestedVersion::Path(semver::VersionReq::parse("1.2").unwrap()),
        RequestedVersion::Yard("1.2".into()),
        RequestedVersion::Yard("1.1".into()),
      ]);
      let have = versions.largest_yard();
      let want = Version::from("1.2");
      assert_eq!(have, Some(&want));
    }

    #[test]
    fn system_no_versions() {
      let versions = RequestedVersions::from(vec![RequestedVersion::Path(semver::VersionReq::parse("1.2").unwrap())]);
      let have = versions.largest_yard();
      assert_eq!(have, None);
    }

    #[test]
    fn empty() {
      let versions = RequestedVersions::new();
      let have = versions.largest_yard();
      assert_eq!(have, None);
    }
  }

  mod update_largest_with {
    use crate::configuration::{RequestedVersion, RequestedVersions, Version};

    #[test]
    fn system_and_versions() {
      let mut versions = RequestedVersions::from(vec![
        RequestedVersion::Path(semver::VersionReq::parse("1.2").unwrap()),
        RequestedVersion::Yard("1.2".into()),
        RequestedVersion::Yard("1.1".into()),
      ]);
      let have = versions.update_largest_with(&Version::from("1.4"));
      assert_eq!(have, Some(Version::from("1.2")));
      let want = RequestedVersions::from(vec![
        RequestedVersion::Path(semver::VersionReq::parse("1.2").unwrap()),
        RequestedVersion::Yard("1.4".into()),
        RequestedVersion::Yard("1.1".into()),
      ]);
      assert_eq!(versions, want);
    }

    #[test]
    fn system_only() {
      let mut versions = RequestedVersions::from(vec![RequestedVersion::Path(semver::VersionReq::parse("1.2").unwrap())]);
      let have = versions.update_largest_with(&Version::from("1.4"));
      assert_eq!(have, None);
      let want = RequestedVersions::from(vec![RequestedVersion::Path(semver::VersionReq::parse("1.2").unwrap())]);
      assert_eq!(versions, want);
    }
  }
}
