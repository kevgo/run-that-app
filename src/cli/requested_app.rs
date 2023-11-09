use crate::error::UserError;
use crate::Result;

/// a request from the user to run a particular app
#[derive(Debug, PartialEq)]
pub struct RequestedApp {
    pub name: String,
    pub version: String,
}

pub fn parse(token: &str) -> Result<RequestedApp> {
    let (app_name, version) = token.split_once('@').unwrap_or((token, ""));
    if version.is_empty() {
        return Err(UserError::RunRequestMissingVersion);
    }
    Ok(RequestedApp {
        name: app_name.to_string(),
        version: version.to_string(),
    })
}

#[cfg(test)]
mod tests {
    mod parse {
        use crate::cli::requested_app;
        use crate::cli::RequestedApp;
        use crate::error::UserError;
        use big_s::S;

        #[test]
        fn name_and_version() {
            let give = "shellcheck@0.9.0";
            let have = requested_app::parse(give);
            let want = Ok(RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_only() {
            let give = "shellcheck";
            let have = requested_app::parse(give);
            let want = Err(UserError::RunRequestMissingVersion);
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn empty_version() {
            let give = "shellcheck@";
            let have = requested_app::parse(give);
            let want = Err(UserError::RunRequestMissingVersion);
            pretty::assert_eq!(have, want);
        }
    }
}
