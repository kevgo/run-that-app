use super::Version;
use crate::apps::App;
use crate::error::UserError;
use crate::Result;
use std::fmt::Display;

/// an application version requested by the user
#[derive(Clone, Debug, PartialEq)]
pub enum RequestedVersion {
    /// the user has requested an externally installed application that matches the given version requirement
    Path(semver::VersionReq),
    /// the user has requested an application in the Yard with the exact version given
    Yard(Version),
}

impl RequestedVersion {
    pub fn parse(version: &str, app: &dyn App) -> Result<RequestedVersion> {
        if let Some(system_version) = is_system(version) {
            if system_version == "auto" {
                return Ok(RequestedVersion::Path(app.allowed_versions()?));
            }
            let version_req = semver::VersionReq::parse(&system_version).map_err(|err| UserError::CannotParseSemverRange {
                expression: system_version.to_string(),
                reason: err.to_string(),
            })?;
            return Ok(RequestedVersion::Path(version_req));
        }
        Ok(RequestedVersion::Yard(version.into()))
    }
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

    mod parse {
        use crate::apps::{AnalyzeResult, App};
        use crate::config::Version;
        use crate::output::Output;
        use crate::platform::Platform;
        use crate::subshell::Executable;
        use crate::yard::Yard;
        use crate::Result;

        /// an App instance that allows to mock the system version restrictions
        struct TestApp {
            allowed_versions: semver::VersionReq,
        }
        impl App for TestApp {
            fn allowed_versions(&self) -> Result<semver::VersionReq> {
                Ok(self.allowed_versions.clone())
            }

            fn name(&self) -> crate::config::AppName {
                unimplemented!()
            }
            fn executable_filename(&self, _platform: Platform) -> &'static str {
                unimplemented!()
            }
            fn executable_filepath(&self, _platform: Platform) -> &'static str {
                unimplemented!()
            }
            fn homepage(&self) -> &'static str {
                unimplemented!()
            }
            fn install(&self, _version: &Version, _platform: Platform, _yard: &Yard, _output: &dyn Output) -> Result<Option<Executable>> {
                unimplemented!()
            }
            fn load(&self, _version: &Version, _platform: Platform, _yard: &Yard) -> Option<Executable> {
                unimplemented!()
            }
            fn installable_versions(&self, _amount: usize, _output: &dyn Output) -> Result<Vec<Version>> {
                unimplemented!()
            }
            fn latest_installable_version(&self, _output: &dyn Output) -> Result<Version> {
                unimplemented!()
            }
            fn analyze_executable(&self, _path: &Executable) -> AnalyzeResult {
                unimplemented!()
            }
        }

        mod unknown_allowed_versions {
            use crate::config::RequestedVersion;

            #[test]
            fn system_request_with_version() {
                let app = super::TestApp {
                    allowed_versions: semver::VersionReq::STAR,
                };
                let have = RequestedVersion::parse("system@1.2", &app).unwrap();
                let want = RequestedVersion::Path(semver::VersionReq::parse("1.2").unwrap());
                assert_eq!(have, want);
            }

            #[test]
            fn system_request_auto_version() {
                let app = super::TestApp {
                    allowed_versions: semver::VersionReq::STAR,
                };
                let have = RequestedVersion::parse("system@auto", &app).unwrap();
                let want = RequestedVersion::Path(semver::VersionReq::STAR);
                assert_eq!(have, want);
            }
        }

        mod known_allowed_versions {
            use crate::config::RequestedVersion;

            #[test]
            fn system_request_with_version() {
                let app = super::TestApp {
                    allowed_versions: semver::VersionReq::parse("1.21").unwrap(),
                };
                let have = RequestedVersion::parse("system@1.5", &app).unwrap();
                let want = RequestedVersion::Path(semver::VersionReq::parse("1.5").unwrap());
                assert_eq!(have, want);
            }

            #[test]
            fn system_request_auto_version() {
                let app = super::TestApp {
                    allowed_versions: semver::VersionReq::parse("1.21").unwrap(),
                };
                let have = RequestedVersion::parse("system@auto", &app).unwrap();
                let want = RequestedVersion::Path(semver::VersionReq::parse("1.21").unwrap());
                assert_eq!(have, want);
            }
        }
    }

    #[test]
    fn is_system() {
        assert_eq!(super::is_system("system@1.2"), Some(S("1.2")));
        assert_eq!(super::is_system("system"), Some(S("*")));
        assert_eq!(super::is_system("1.2.3"), None);
    }
}
