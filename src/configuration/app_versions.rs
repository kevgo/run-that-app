use super::{RequestedVersion, RequestedVersions};
use crate::applications::ApplicationName;

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
    // Sort first by app name
    match self.app_name.as_str().cmp(other.app_name.as_str()) {
      std::cmp::Ordering::Equal => {
        // Then sort by versions
        // Compare length first
        match self.versions.0.len().cmp(&other.versions.0.len()) {
          std::cmp::Ordering::Equal => {
            // If same length, compare element by element
            for (a, b) in self.versions.0.iter().zip(other.versions.0.iter()) {
              let cmp = match (a, b) {
                (RequestedVersion::Yard(v1), RequestedVersion::Yard(v2)) => {
                  // Use PartialOrd, fallback to string comparison if PartialOrd returns None
                  v1.partial_cmp(v2).unwrap_or_else(|| v1.as_str().cmp(v2.as_str()))
                }
                (RequestedVersion::Path(v1), RequestedVersion::Path(v2)) => v1.to_string().cmp(&v2.to_string()),
                // Path comes before Yard in ordering
                (RequestedVersion::Path(_), RequestedVersion::Yard(_)) => std::cmp::Ordering::Less,
                (RequestedVersion::Yard(_), RequestedVersion::Path(_)) => std::cmp::Ordering::Greater,
              };
              if cmp != std::cmp::Ordering::Equal {
                return cmp;
              }
            }
            std::cmp::Ordering::Equal
          }
          other => other,
        }
      }
      other => other,
    }
  }
}
