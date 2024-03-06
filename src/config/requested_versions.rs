use super::{AppName, Config, RequestedVersion, Version};
use crate::error::UserError;
use crate::Result;

/// a collection of Version instances
#[derive(Debug, PartialEq)]
pub struct RequestedVersions(Vec<RequestedVersion>);

impl RequestedVersions {
    /// Provides the version to use: if the user provided a version to use via CLI, use it.
    /// Otherwise provide the versions from the config file.
    pub fn determine(app: &AppName, cli_version: Option<Version>) -> Result<RequestedVersions> {
        if let Some(version) = cli_version {
            return Ok(RequestedVersions::from(version));
        }
        match Config::load()?.lookup(app) {
            Some(versions) => Ok(versions),
            None => Err(UserError::RunRequestMissingVersion),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, RequestedVersion> {
        self.0.iter()
    }

    pub fn join(&self, sep: &str) -> String {
        let strings: Vec<String> = self.0.iter().map(RequestedVersion::to_string).collect();
        strings.join(sep)
    }

    /// provides the largest yard version contained in this collection
    /// TODO: rename to `largest_yard`
    fn largest_non_system(&self) -> Option<&Version> {
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

    pub fn push(&mut self, value: RequestedVersion) {
        self.0.push(value);
    }

    /// Updates the largest non-system version in this collection with the given value.
    /// Returns the value that was replaced.
    pub fn update_largest_with(&mut self, value: &Version) -> Option<Version> {
        let Some(largest) = self.largest_non_system() else {
            return None;
        };
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

impl From<RequestedVersion> for RequestedVersions {
    fn from(requested_version: RequestedVersion) -> Self {
        RequestedVersions(vec![requested_version])
    }
}

impl From<Version> for RequestedVersions {
    fn from(version: Version) -> Self {
        RequestedVersions(vec![RequestedVersion::from(version)])
    }
}

impl From<&str> for RequestedVersions {
    fn from(version: &str) -> Self {
        RequestedVersions::from(RequestedVersion::from(version))
    }
}

impl From<Vec<&str>> for RequestedVersions {
    fn from(versions: Vec<&str>) -> Self {
        let versions = versions.into_iter().map(RequestedVersion::from).collect();
        RequestedVersions(versions)
    }
}

#[cfg(test)]
mod tests {

    mod join {
        use crate::config::RequestedVersions;

        #[test]
        fn multiple() {
            let versions = RequestedVersions::from(vec!["system@1.2", "1.2", "1.1"]);
            let have = versions.join(", ");
            let want = "system@1.2, 1.2, 1.1";
            assert_eq!(have, want);
        }

        #[test]
        fn one() {
            let versions = RequestedVersions::from(vec!["system@1.2"]);
            let have = versions.join(", ");
            let want = "system@1.2";
            assert_eq!(have, want);
        }

        #[test]
        fn zero() {
            let versions = RequestedVersions::from(vec![]);
            let have = versions.join(", ");
            let want = "";
            assert_eq!(have, want);
        }
    }

    mod largest_non_system {
        use crate::config::{RequestedVersions, Version};

        #[test]
        fn system_and_versions() {
            let versions = RequestedVersions::from(vec!["system@1.2", "1.2", "1.1"]);
            let have = versions.largest_non_system();
            let want = Version::from("1.2");
            assert_eq!(have, Some(&want));
        }

        #[test]
        fn system_no_versions() {
            let versions = RequestedVersions::from(vec!["system@1.2"]);
            let have = versions.largest_non_system();
            assert_eq!(have, None);
        }

        #[test]
        fn empty() {
            let versions = RequestedVersions::from(vec![]);
            let have = versions.largest_non_system();
            assert_eq!(have, None);
        }
    }

    mod update_largest_with {
        use crate::config::{RequestedVersions, Version};

        #[test]
        fn system_and_versions() {
            let mut versions = RequestedVersions::from(vec!["system@1.2", "1.2", "1.1"]);
            let have = versions.update_largest_with(&Version::from("1.4"));
            assert_eq!(have, Some(Version::from("1.2")));
            let want = RequestedVersions::from(vec!["system@1.2", "1.4", "1.1"]);
            assert_eq!(versions, want);
        }

        #[test]
        fn system_only() {
            let mut versions = RequestedVersions::from(vec!["system@1.2"]);
            let have = versions.update_largest_with(&Version::from("1.4"));
            assert_eq!(have, None);
            let want = RequestedVersions::from(vec!["system@1.2"]);
            assert_eq!(versions, want);
        }
    }
}
