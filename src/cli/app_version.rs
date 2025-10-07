use crate::applications::{ApplicationName, Apps};
use crate::configuration::Version;
use crate::error::Result;

/// a request from the user to run a particular app
#[derive(Debug, PartialEq)]
pub(crate) struct AppVersion {
  pub(crate) app_name: ApplicationName,
  pub(crate) version: Option<Version>,
}

impl AppVersion {
  pub(crate) fn new<S: AsRef<str>>(token: S, apps: &Apps) -> Result<Self> {
    let (app_name, version) = token.as_ref().split_once('@').unwrap_or((token.as_ref(), ""));
    let app = apps.lookup(app_name)?;
    let version = if version.is_empty() { None } else { Some(Version::from(version)) };
    let app_name = app.name();
    Ok(AppVersion { app_name, version })
  }
}

#[cfg(test)]
mod tests {
  mod parse {
    use crate::applications;
    use crate::cli::AppVersion;
    use crate::configuration::Version;

    #[test]
    fn name_and_version() {
      let give = "shellcheck@0.9.0";
      let apps = applications::all();
      let have = AppVersion::new(give, &apps);
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let want = Ok(AppVersion {
        app_name: shellcheck.name(),
        version: Some(Version::from("0.9.0")),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn name_only() {
      let give = "shellcheck";
      let apps = applications::all();
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let have = AppVersion::new(give, &apps);
      let want = Ok(AppVersion {
        app_name: shellcheck.name(),
        version: None,
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn empty_version() {
      let give = "shellcheck@";
      let apps = applications::all();
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let have = AppVersion::new(give, &apps);
      let want = Ok(AppVersion {
        app_name: shellcheck.name(),
        version: None,
      });
      pretty::assert_eq!(have, want);
    }
  }
}
