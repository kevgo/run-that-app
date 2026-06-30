use crate::applications::{AppDefinition, Apps};
use crate::configuration::Version;
use crate::error::Result;
use std::fmt::Debug;

/// a request from the user to run a particular app
pub struct AppVersion<'a> {
  pub app: &'a Box<dyn AppDefinition>,
  pub version: Option<Version>,
}

impl Debug for AppVersion<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("AppVersion")
      .field("app", &self.app.name())
      .field("version", &self.version)
      .finish()
  }
}

impl PartialEq for AppVersion<'_> {
  fn eq(&self, other: &Self) -> bool {
    self.app.name() == other.app.name() && self.version == other.version
  }
}

impl<'a> AppVersion<'a> {
  pub fn parse<S: AsRef<str>>(token: S, apps: &'a Apps) -> Result<Self> {
    let (app_name, version) = token.as_ref().split_once('@').unwrap_or((token.as_ref(), ""));
    let app = apps.lookup(app_name)?;
    let version = if version.is_empty() { None } else { Some(Version::from(version)) };
    Ok(AppVersion { app, version })
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
      let have = AppVersion::parse(give, &apps);
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let want = Ok(AppVersion {
        app: shellcheck,
        version: Some(Version::from("0.9.0")),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn name_only() {
      let give = "shellcheck";
      let apps = applications::all();
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let have = AppVersion::parse(give, &apps);
      let want = Ok(AppVersion {
        app: shellcheck,
        version: None,
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn empty_version() {
      let give = "shellcheck@";
      let apps = applications::all();
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let have = AppVersion::parse(give, &apps);
      let want = Ok(AppVersion {
        app: shellcheck,
        version: None,
      });
      pretty::assert_eq!(have, want);
    }
  }
}
