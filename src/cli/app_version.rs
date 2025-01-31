use crate::applications::ApplicationName;
use crate::configuration::Version;

/// a request from the user to run a particular app
#[derive(Debug, PartialEq)]
pub(crate) struct AppVersion {
  pub(crate) app_name: ApplicationName,
  pub(crate) version: Option<Version>,
}

impl AppVersion {
  pub(crate) fn new<S: AsRef<str>>(token: S) -> Self {
    let (app_name, version) = token.as_ref().split_once('@').unwrap_or((token.as_ref(), ""));
    let version = if version.is_empty() { None } else { Some(Version::from(version)) };
    AppVersion {
      app_name: app_name.into(),
      version,
    }
  }
}

#[cfg(test)]
mod tests {
  mod parse {
    use crate::applications::ApplicationName;
    use crate::cli::AppVersion;
    use crate::configuration::Version;

    #[test]
    fn name_and_version() {
      let give = "shellcheck@0.9.0";
      let have = AppVersion::new(give);
      let want = AppVersion {
        app_name: ApplicationName::from("shellcheck"),
        version: Some(Version::from("0.9.0")),
      };
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn name_only() {
      let give = "shellcheck";
      let have = AppVersion::new(give);
      let want = AppVersion {
        app_name: ApplicationName::from("shellcheck"),
        version: None,
      };
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn empty_version() {
      let give = "shellcheck@";
      let have = AppVersion::new(give);
      let want = AppVersion {
        app_name: ApplicationName::from("shellcheck"),
        version: None,
      };
      pretty::assert_eq!(have, want);
    }
  }
}
