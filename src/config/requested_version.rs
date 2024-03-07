use super::Version;
use crate::error::UserError;
use std::fmt::Display;

/// an application version requested by the user
#[derive(Clone, Debug, PartialEq)]
pub enum RequestedVersion {
    /// the user has requested an externally installed application that matches the given version requirement
    Path(semver::VersionReq),
    /// the user has requested an application in the Yard with the exact version given
    Yard(Version),
}

impl Display for RequestedVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestedVersion::Path(version) => {
                f.write_str("system@")?;
                f.write_str(&version.to_string())
            }
            RequestedVersion::Yard(version) => f.write_str(version.as_str()),
        }
    }
}

impl From<Version> for RequestedVersion {
    fn from(value: Version) -> Self {
        RequestedVersion::Yard(value)
    }
}

impl TryFrom<&str> for RequestedVersion {
    type Error = UserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(version_req) = is_system(value) {
            let version_req = semver::VersionReq::parse(&version_req).map_err(|err| UserError::CannotParseSemverRange {
                expression: version_req.to_string(),
                reason: err.to_string(),
            })?;
            Ok(RequestedVersion::Path(version_req))
        } else {
            Ok(RequestedVersion::Yard(value.into()))
        }
    }
}

/// Indicates whether the given version string requests an executable in the PATH or in the yard.
/// Also provides the sanitized version string without the "system" prefix.
fn is_system(value: &str) -> Option<String> {
    if value.starts_with("system@") {
        return value.strip_prefix("system@").map(ToString::to_string);
    }
    if value == "system" {
        return Some(String::from("*"));
    }
    None
}

#[cfg(test)]
mod tests {
    use big_s::S;

    mod from {
        use crate::config::RequestedVersion;

        #[test]
        fn system_request() {
            let have = RequestedVersion::try_from("system@1.2").unwrap();
            let want = RequestedVersion::Path(semver::VersionReq::parse("1.2").unwrap());
            assert_eq!(have, want);
        }
    }

    #[test]
    fn is_system() {
        assert_eq!(super::is_system("system@1.2"), Some(S("1.2")));
        assert_eq!(super::is_system("system"), Some(S("*")));
        assert_eq!(super::is_system("1.2.3"), None);
    }
}
