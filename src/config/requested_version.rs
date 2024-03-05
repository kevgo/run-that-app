use super::Version;

/// an application version requested by the user
#[derive(Clone, Debug, PartialEq)]
pub enum RequestedVersion {
    /// the user has requested an externally installed application that matches the given version requirement
    Path(String),
    /// the user has requested an application in the Yard with the exact version given
    Yard(Version),
}

impl RequestedVersion {
    pub fn as_str(&self) -> &str {
        match self {
            RequestedVersion::Path(version) => &version,
            RequestedVersion::Yard(version) => version.as_str(),
        }
    }
}

impl From<Version> for RequestedVersion {
    fn from(value: Version) -> Self {
        RequestedVersion::Yard(value)
    }
}

impl From<&str> for RequestedVersion {
    fn from(value: &str) -> Self {
        if is_system(value) {
            RequestedVersion::Path(value.into())
        } else {
            RequestedVersion::Yard(value.into())
        }
    }
}

fn is_system(value: &str) -> bool {
    value.starts_with("system@") || value == "system"
}
