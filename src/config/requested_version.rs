use std::fmt::Display;

use super::Version;

/// an application version requested by the user
#[derive(Clone, Debug, PartialEq)]
pub enum RequestedVersion {
    /// the user has requested an externally installed application that matches the given version requirement
    Path(String),
    /// the user has requested an application in the Yard with the exact version given
    Yard(Version),
}

impl Display for RequestedVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestedVersion::Path(version) => {
                f.write_str("system@")?;
                f.write_str(version.as_str())
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

impl From<&str> for RequestedVersion {
    fn from(value: &str) -> Self {
        if let Some(version_req) = is_system(value) {
            RequestedVersion::Path(version_req)
        } else {
            RequestedVersion::Yard(value.into())
        }
    }
}

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
        use big_s::S;

        #[test]
        fn system_request() {
            let have = RequestedVersion::from("system@1.2");
            let want = RequestedVersion::Path(S("1.2"));
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
