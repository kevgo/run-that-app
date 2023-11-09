/// a request from the user to run a particular app
#[derive(Debug, PartialEq)]
pub struct RequestedApp {
    pub name: String,
    pub version: String,
}

pub fn parse(token: &str) -> RequestedApp {
    let (app_name, version) = token.split_once('@').unwrap_or((token, ""));
    RequestedApp {
        name: app_name.to_string(),
        version: version.to_string(),
    }
}

#[cfg(test)]
mod tests {
    mod parse {
        use crate::ui::cli_args::requested_app;
        use crate::ui::RequestedApp;
        use big_s::S;

        #[test]
        fn name_and_version() {
            let give = "shellcheck@0.9.0";
            let have = requested_app::parse(give);
            let want = RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_only() {
            let give = "shellcheck";
            let have = requested_app::parse(give);
            let want = RequestedApp {
                name: S("shellcheck"),
                version: S(""),
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn empty_version() {
            let give = "shellcheck@";
            let have = requested_app::parse(give);
            let want = RequestedApp {
                name: S("shellcheck"),
                version: S(""),
            };
            pretty::assert_eq!(have, want);
        }
    }
}
