use std::cmp::Ordering;
use std::fmt::Display;
use std::path::Path;

/// a string that represents
#[derive(Clone, Debug, PartialEq)]
pub struct Version(String);

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(semver_order) = compare_semver(self, other) {
            return Some(semver_order);
        }
        self.as_str().partial_cmp(other.as_str())
    }
}

fn compare_semver(v1: &Version, v2: &Version) -> Option<Ordering> {
    let Ok(self_version) = semver::Version::parse(v1.as_str()) else {
        return None;
    };
    let Ok(other_version) = semver::Version::parse(v2.as_str()) else {
        return None;
    };
    self_version.partial_cmp(&other_version)
}

impl Version {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn is_system(&self) -> bool {
        self.0.starts_with("system@") || self.0 == "system"
    }
}

impl AsRef<Path> for Version {
    fn as_ref(&self) -> &Path {
        let text: &str = self.as_str();
        Path::new(text)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for Version {
    fn from(text: &str) -> Self {
        Version(text.to_string())
    }
}

impl From<String> for Version {
    fn from(text: String) -> Self {
        Version(text)
    }
}

impl PartialEq<str> for Version {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<String> for Version {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == *other
    }
}

#[cfg(test)]
mod tests {

    use crate::config::Version;

    #[test]
    fn is_system() {
        assert!(Version::from("system").is_system());
        assert!(Version::from("system@1.2").is_system());
        assert!(!Version::from("1.2.3").is_system());
    }

    mod partial_cmp {
        use crate::config::Version;

        #[test]
        fn semantic() {
            let version = Version::from("3.10.2");
            let other = Version::from("3.2.1");
            assert!(version > other);
        }

        #[test]
        fn tag() {
            let version = Version::from("1.2.3-alpha");
            let other = Version::from("1.2.3");
            assert!(version < other);
        }
    }
}
