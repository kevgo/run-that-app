/// a request from the user to run a particular app
#[derive(Debug, PartialEq)]
pub struct AppVersion {
    pub name: String,
    pub version: String,
}

pub fn parse(token: &str) -> AppVersion {
    let (app_name, version) = token.split_once('@').unwrap_or((token, ""));
    AppVersion {
        name: app_name.to_string(),
        version: version.to_string(),
    }
}

#[cfg(test)]
mod tests {
    mod parse {
        use crate::cli::app_version;
        use crate::cli::AppVersion;
        use big_s::S;

        #[test]
        fn name_and_version() {
            let give = "shellcheck@0.9.0";
            let have = app_version::parse(give);
            let want = AppVersion {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_only() {
            let give = "shellcheck";
            let have = app_version::parse(give);
            let want = AppVersion {
                name: S("shellcheck"),
                version: S(""),
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn empty_version() {
            let give = "shellcheck@";
            let have = app_version::parse(give);
            let want = AppVersion {
                name: S("shellcheck"),
                version: S(""),
            };
            pretty::assert_eq!(have, want);
        }
    }
}
