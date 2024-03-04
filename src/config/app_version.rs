use super::{AppName, Version};

/// a request from the user to run a particular app
#[derive(Debug, PartialEq)]
pub struct AppVersion {
    pub app: AppName,
    pub version: Option<Version>,
}

impl AppVersion {
    pub fn new<S: AsRef<str>>(token: S) -> Self {
        let (app_name, version) = token.as_ref().split_once('@').unwrap_or((token.as_ref(), ""));
        AppVersion {
            app: app_name.into(),
            version: version.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod parse {
        use crate::config::{AppName, AppVersion, Version};

        #[test]
        fn name_and_version() {
            let give = "shellcheck@0.9.0";
            let have = AppVersion::new(give);
            let want = AppVersion {
                app: AppName::from("shellcheck"),
                version: Version::from("0.9.0"),
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_only() {
            let give = "shellcheck";
            let have = AppVersion::new(give);
            let want = AppVersion {
                app: AppName::from("shellcheck"),
                version: Version::None,
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn empty_version() {
            let give = "shellcheck@";
            let have = AppVersion::new(give);
            let want = AppVersion {
                app: AppName::from("shellcheck"),
                version: Version::None,
            };
            pretty::assert_eq!(have, want);
        }
    }
}
