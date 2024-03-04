use std::cmp::Ordering;
use std::fmt::Display;
use std::path::Path;

/// a string that represents
#[derive(Debug, PartialEq)]
pub struct Version(String);

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // TODO: compare each version number element (major, minor) using human-sort
        // maybe the semver crate has a comp function that we can use here?
        match (self, other) {
            (Version::Some(this), Version::Some(other)) => this.partial_cmp(other),
            (Version::Some(_), Version::None) => Some(Ordering::Greater),
            (Version::None, Version::Some(_)) => Some(Ordering::Less),
            (Version::None, Version::None) => Some(Ordering::Equal),
        }
    }
}

impl Version {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
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
