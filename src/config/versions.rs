use super::{AppName, Config, Version};
use crate::error::UserError;
use crate::Result;

/// a collection of Version instances
#[derive(Debug, PartialEq)]
pub struct Versions(Vec<Version>);

impl Versions {
    /// Provides the version to use: if the user provided a version to use via CLI, use it.
    /// Otherwise provide the versions from the config file.
    pub fn determine(app: &AppName, cli_version: Option<Version>) -> Result<Versions> {
        if let Some(version) = cli_version {
            return Ok(Versions::from(version));
        }
        match Config::load()?.lookup(app) {
            Some(versions) => Ok(versions),
            None => Err(UserError::RunRequestMissingVersion),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Version> {
        self.0.iter()
    }

    pub fn join(&self, sep: &str) -> String {
        let strings: Vec<&str> = self.0.iter().map(Version::as_str).collect();
        strings.join(sep)
    }

    /// provides the largest non-system version contained in this collection
    fn largest_non_system(&self) -> Option<&Version> {
        let mut result = None;
        for version in &self.0 {
            if version.is_system() {
                continue;
            }
            match result {
                Some(max) if version > max => result = Some(version),
                Some(_) => {}
                None => result = Some(version),
            }
        }
        result
    }

    pub fn push(&mut self, value: Version) {
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
            if self.0[i] == largest {
                updated = Some(self.0[i].clone());
                self.0[i] = value.clone();
            }
        }
        updated
    }
}

impl From<Version> for Versions {
    fn from(version: Version) -> Self {
        Versions(vec![version])
    }
}

impl From<&str> for Versions {
    fn from(version: &str) -> Self {
        Versions::from(Version::from(version))
    }
}

impl From<Vec<&str>> for Versions {
    fn from(versions: Vec<&str>) -> Self {
        let versions = versions.into_iter().map(Version::from).collect();
        Versions(versions)
    }
}

#[cfg(test)]
mod tests {

    mod join {
        use crate::config::Versions;

        #[test]
        fn multiple() {
            let versions = Versions::from(vec!["system@1.2", "1.2", "1.1"]);
            let have = versions.join(", ");
            let want = "system@1.2, 1.2, 1.1";
            assert_eq!(have, want);
        }

        #[test]
        fn one() {
            let versions = Versions::from(vec!["system@1.2"]);
            let have = versions.join(", ");
            let want = "system@1.2";
            assert_eq!(have, want);
        }

        #[test]
        fn zero() {
            let versions = Versions::from(vec![]);
            let have = versions.join(", ");
            let want = "";
            assert_eq!(have, want);
        }
    }

    mod largest_non_system {
        use crate::config::{Version, Versions};

        #[test]
        fn system_and_versions() {
            let versions = Versions::from(vec!["system@1.2", "1.2", "1.1"]);
            let have = versions.largest_non_system();
            let want = Version::from("1.2");
            assert_eq!(have, Some(&want));
        }

        #[test]
        fn system_no_versions() {
            let versions = Versions::from(vec!["system@1.2"]);
            let have = versions.largest_non_system();
            assert_eq!(have, None);
        }

        #[test]
        fn empty() {
            let versions = Versions::from(vec![]);
            let have = versions.largest_non_system();
            assert_eq!(have, None);
        }
    }

    mod update_largest_with {
        use crate::config::{Version, Versions};

        #[test]
        fn system_and_versions() {
            let mut versions = Versions::from(vec!["system@1.2", "1.2", "1.1"]);
            let have = versions.update_largest_with(&Version::from("1.4"));
            assert_eq!(have, Some(Version::from("1.2")));
            let want = Versions::from(vec!["system@1.2", "1.4", "1.1"]);
            assert_eq!(versions, want);
        }

        #[test]
        fn system_only() {
            let mut versions = Versions::from(vec!["system@1.2"]);
            let have = versions.update_largest_with(&Version::from("1.4"));
            assert_eq!(have, None);
            let want = Versions::from(vec!["system@1.2"]);
            assert_eq!(versions, want);
        }
    }
}
