use super::{AppVersions, FILE_NAME, RequestedVersion, RequestedVersions, Version};
use crate::applications::{ApplicationName, Apps};
use crate::filesystem;
use crate::error::{Result, UserError};
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::str::SplitAsciiWhitespace;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct File {
  pub(crate) apps: Vec<AppVersions>,
}

impl File {
  pub(crate) fn add(mut self, app_name: ApplicationName, version: Version) -> Result<()> {
    self.apps.push(AppVersions {
      app_name,
      versions: RequestedVersions::from(vec![RequestedVersion::Yard(version)]),
    });
    self.apps.sort();
    self.save()
  }

  pub(crate) fn create(app: &ApplicationName, version: &Version) -> Result<()> {
    let mut file = match OpenOptions::new().write(true).create_new(true).open(FILE_NAME) {
      Ok(file) => file,
      Err(err) => {
        if err.kind() == ErrorKind::AlreadyExists {
          return Err(UserError::ConfigFileAlreadyExists);
        }
        return Err(UserError::CannotCreateFile {
          filename: FILE_NAME,
          err: err.to_string(),
        });
      }
    };
    let content = format!("{app} {version}");
    file
      .write_all(content.as_bytes())
      .map_err(|err| UserError::CannotAccessConfigFile(err.to_string()))
  }

  pub(crate) fn read(apps: &Apps) -> Result<Option<File>> {
    match filesystem::read_file(FILE_NAME)? {
      Some(text) => Ok(Some(parse(&text, apps)?)),
      None => Ok(None),
    }
  }

  pub(crate) fn load(apps: &Apps) -> Result<File> {
    Ok(Self::read(apps)?.unwrap_or_default())
  }

  pub(crate) fn lookup(&self, app_name: &ApplicationName) -> Option<&RequestedVersions> {
    self.apps.iter().find(|app| &app.app_name == app_name).map(|app_version| &app_version.versions)
  }

  pub(crate) fn lookup_many(&self, app_names: Vec<ApplicationName>) -> Vec<AppVersions> {
    let mut result = vec![];
    for app_name in app_names {
      if let Some(versions) = self.lookup(&app_name) {
        result.push(AppVersions {
          app_name,
          versions: versions.into(),
        });
      }
    }
    result
  }

  pub(crate) fn save(&self) -> Result<()> {
    let mut file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(FILE_NAME)
      .map_err(|err| UserError::CannotAccessConfigFile(err.to_string()))?;
    file
      .write_all(self.to_string().as_bytes())
      .map_err(|err| UserError::CannotAccessConfigFile(err.to_string()))?;
    Ok(())
  }
}

impl Display for File {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for AppVersions { app_name, versions } in &self.apps {
      f.write_str(app_name.as_str())?;
      for version in versions {
        f.write_str(" ")?;
        version.fmt(f)?;
      }
      f.write_str("\n")?;
    }
    Ok(())
  }
}

fn parse(text: &str, all_apps: &Apps) -> Result<File> {
  let mut apps = vec![];
  for (i, line) in text.lines().enumerate() {
    if let Some(app_version) = parse_line(line, i, all_apps)? {
      apps.push(app_version);
    }
  }
  Ok(File { apps })
}

fn parse_line(line_text: &str, line_no: usize, apps: &Apps) -> Result<Option<AppVersions>> {
  let line_text = line_text.trim();
  let mut parts = LinePartsIterator::from(line_text);
  let Some(name) = parts.next() else {
    // empty or commented out line --> ignore
    return Ok(None);
  };
  let app = apps.lookup(name)?;
  let Some(version) = parts.next() else {
    // line has only one element --> invalid
    return Err(UserError::InvalidConfigFileFormat {
      line_no,
      text: line_text.to_string(),
    });
  };
  let mut versions = RequestedVersions::from(vec![RequestedVersion::parse(version, app)?]);
  for part in parts {
    versions.push(RequestedVersion::parse(part, app)?);
  }
  Ok(Some(AppVersions {
    app_name: app.app_name(),
    versions,
  }))
}

/// provides active (non-comment) words in the given line
struct LinePartsIterator<'a> {
  parts: SplitAsciiWhitespace<'a>,
}

impl<'a> From<&'a str> for LinePartsIterator<'a> {
  fn from(line: &'a str) -> Self {
    LinePartsIterator {
      parts: line.split_ascii_whitespace(),
    }
  }
}

impl<'a> Iterator for LinePartsIterator<'a> {
  type Item = &'a str;

  fn next(&mut self) -> Option<Self::Item> {
    let part = self.parts.next()?;
    if part.starts_with('#') {
      return None;
    }
    Some(part)
  }
}

#[cfg(test)]
mod tests {

  mod parse {
    use super::super::parse;
    use crate::applications;
    use crate::configuration::{self, AppVersions, RequestedVersion, RequestedVersions};

    #[test]
    fn normal() {
      let give = "actionlint 1.2.3\n\
                        dprint  2.3.4 # comment\n\
                        mdbook 3.4.5 6.7.8\n\
                        go system@1.21 1.22.1";
      let apps = applications::all();
      let actionlint = apps.lookup("actionlint").unwrap();
      let dprint = apps.lookup("dprint").unwrap();
      let mdbook = apps.lookup("mdbook").unwrap();
      let go = apps.lookup("go").unwrap();
      let have = parse(give, &apps).unwrap();
      let want = configuration::File {
        apps: vec![
          AppVersions {
            app_name: actionlint.app_name(),
            versions: RequestedVersions::from(vec![RequestedVersion::Yard("1.2.3".into())]),
          },
          AppVersions {
            app_name: dprint.app_name(),
            versions: RequestedVersions::from(vec![RequestedVersion::Yard("2.3.4".into())]),
          },
          AppVersions {
            app_name: mdbook.app_name(),
            versions: RequestedVersions::from(vec![RequestedVersion::Yard("3.4.5".into()), RequestedVersion::Yard("6.7.8".into())]),
          },
          AppVersions {
            app_name: go.app_name(),
            versions: RequestedVersions::from(vec![
              RequestedVersion::Path(semver::VersionReq::parse("1.21").unwrap()),
              RequestedVersion::Yard("1.22.1".into()),
            ]),
          },
        ],
      };
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn empty() {
      let give = "";
      let apps = applications::all();
      let have = parse(give, &apps).unwrap();
      let want = configuration::File { apps: vec![] };
      pretty::assert_eq!(have, want);
    }
  }

  mod parse_line {
    use super::super::parse_line;
    use crate::applications;
    use crate::configuration::{AppVersions, RequestedVersion, RequestedVersions};
    use crate::error::UserError;
    use big_s::S;

    #[test]
    fn normal() {
      let apps = applications::all();
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let give = "shellcheck 0.9.0";
      let have = parse_line(give, 1, &apps).unwrap();
      let want = Some(AppVersions {
        app_name: shellcheck.app_name(),
        versions: RequestedVersions::from(vec![RequestedVersion::Yard("0.9.0".into())]),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn multiple_versions() {
      let apps = applications::all();
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let give = "shellcheck 0.9.0 0.6.0";
      let have = parse_line(give, 1, &apps).unwrap();
      let want = Some(AppVersions {
        app_name: shellcheck.app_name(),
        versions: RequestedVersions::from(vec![RequestedVersion::Yard("0.9.0".into()), RequestedVersion::Yard("0.6.0".into())]),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn normal_with_multiple_spaces() {
      let apps = applications::all();
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let give = "     shellcheck            0.9.0      ";
      let have = parse_line(give, 1, &apps).unwrap();
      let want = Some(AppVersions {
        app_name: shellcheck.app_name(),
        versions: RequestedVersions::from(vec![RequestedVersion::Yard("0.9.0".into())]),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn normal_with_tabs() {
      let apps = applications::all();
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let give = "shellcheck\t0.9.0";
      let have = parse_line(give, 1, &apps).unwrap();
      let want = Some(AppVersions {
        app_name: shellcheck.app_name(),
        versions: RequestedVersions::from(vec![RequestedVersion::Yard("0.9.0".into())]),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn missing_version() {
      let apps = applications::all();
      let give = "shellcheck ";
      let have = parse_line(give, 1, &apps);
      let want = Err(UserError::InvalidConfigFileFormat {
        line_no: 1,
        text: S("shellcheck"),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn empty_line() {
      let apps = applications::all();
      let give = "";
      let have = parse_line(give, 1, &apps).unwrap();
      assert_eq!(have, None);
    }

    #[test]
    fn spaces_only() {
      let apps = applications::all();
      let give = "              ";
      let have = parse_line(give, 1, &apps).unwrap();
      assert_eq!(have, None);
    }

    #[test]
    fn completely_commented_out() {
      let apps = applications::all();
      let give = "# shellcheck 0.9.0";
      let have = parse_line(give, 1, &apps).unwrap();
      assert_eq!(have, None);
    }

    #[test]
    fn valid_with_comment_at_end() {
      let apps = applications::all();
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let give = "shellcheck 0.9.0  # comment";
      let have = parse_line(give, 1, &apps).unwrap();
      let want = Some(AppVersions {
        app_name: shellcheck.app_name(),
        versions: RequestedVersions::from(vec![RequestedVersion::Yard("0.9.0".into())]),
      });
      pretty::assert_eq!(have, want);
    }
  }
}
