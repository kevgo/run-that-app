use std::fmt::Display;
use std::path::Path;

/// a string that represents
#[derive(Debug, PartialEq)]
pub enum Version {
    Some(String),
    None,
}

impl Version {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Version::Some(text) => text,
            Version::None => "",
        }
    }

    pub(crate) fn is_none(&self) -> bool {
        match self {
            Version::None => true,
            Version::Some(_) => false,
        }
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
        if let Version::Some(text) = self {
            f.write_str(text)?;
        }
        Ok(())
    }
}

impl From<&str> for Version {
    fn from(text: &str) -> Self {
        if text.is_empty() {
            return Version::None;
        }
        Version::Some(text.to_string())
    }
}

impl From<String> for Version {
    fn from(text: String) -> Self {
        if text.is_empty() {
            return Version::None;
        }
        Version::Some(text)
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