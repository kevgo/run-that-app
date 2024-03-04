use std::cmp::Ordering;
use std::fmt::Display;
use std::path::Path;

/// a string that represents
#[derive(Clone, Debug, PartialEq)]
pub struct Version(String);

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_version = semver::Version::parse(self.as_str()).unwrap();
        let other_version = semver::Version::parse(other.as_str()).unwrap();
        self_version.partial_cmp(&other_version)
    }
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

    mod is_system {
        use crate::config::Version;

        #[test]
        fn tests() {
            check("system", true);
            check("system@1.2", true);
            check("1.2.3", false);
        }

        #[track_caller]
        fn check(give: &str, want: bool) {
            assert_eq!(Version::from(give).is_system(), want);
        }
    }

    mod partial_cmp {
        use crate::config::Version;

        #[test]
        fn semantic() {
            let version = Version::from("3.10.2");
            let other = Version::from("3.2.1");
            assert!(version > other);
        }
    }
}
