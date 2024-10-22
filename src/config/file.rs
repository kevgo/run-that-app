use super::{AppName, AppVersions, RequestedVersion, RequestedVersions, FILE_NAME};
use crate::apps::Apps;
use crate::filesystem;
use crate::prelude::*;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::str::SplitAsciiWhitespace;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct File {
  pub apps: Vec<AppVersions>,
}

impl File {
  pub fn create() -> Result<()> {
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
    let content = "\
# actionlint 1.2.26
# gh 2.39.1
";
    file
      .write_all(content.as_bytes())
      .map_err(|err| UserError::CannotAccessConfigFile(err.to_string()))
  }

  pub fn load(apps: &Apps) -> Result<File> {
    match filesystem::read_file(FILE_NAME)? {
      Some(text) => parse(&text, apps),
      None => Ok(File::default()),
    }
  }

  pub fn lookup(&self, app_name: &AppName) -> Option<&RequestedVersions> {
    self.apps.iter().find(|app| app.app_name == app_name).map(|app_version| &app_version.versions)
  }

  pub fn save(&self) -> Result<()> {
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
      f.write_str(" ")?;
      f.write_str(&versions.join(", "))?;
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
  let app = apps.lookup(&name.into())?;
  let Some(version) = parts.next() else {
    // line has only one element --> invalid
    return Err(UserError::InvalidConfigFileFormat {
      line_no,
      text: line_text.to_string(),
    });
  };
  let mut versions = RequestedVersions::new(vec![RequestedVersion::parse(version, app)?]);
  for part in parts {
    versions.push(RequestedVersion::parse(part, app)?);
  }
  Ok(Some(AppVersions {
    app_name: name.into(),
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
    use crate::apps;
    use crate::config::{self, AppName, AppVersions, RequestedVersion, RequestedVersions};

    #[test]
    fn normal() {
      let give = "actionlint 1.2.3\n\
                        dprint  2.3.4 # comment\n\
                        mdbook 3.4.5 6.7.8\n\
                        go system@1.21 1.22.1";
      let have = parse(give, &apps::all()).unwrap();
      let want = config::File {
        apps: vec![
          AppVersions {
            app_name: "actionlint".into(),
            versions: RequestedVersions::new(vec![RequestedVersion::Yard("1.2.3".into())]),
          },
          AppVersions {
            app_name: AppName::from("dprint"),
            versions: RequestedVersions::new(vec![RequestedVersion::Yard("2.3.4".into())]),
          },
          AppVersions {
            app_name: AppName::from("mdbook"),
            versions: RequestedVersions::new(vec![RequestedVersion::Yard("3.4.5".into()), RequestedVersion::Yard("6.7.8".into())]),
          },
          AppVersions {
            app_name: AppName::from("go"),
            versions: RequestedVersions::new(vec![
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
      let have = parse(give, &apps::all()).unwrap();
      let want = config::File { apps: vec![] };
      pretty::assert_eq!(have, want);
    }
  }

  mod parse_line {
    use super::super::parse_line;
    use crate::apps;
    use crate::config::{AppName, AppVersions, RequestedVersion, RequestedVersions};
    use crate::error::UserError;
    use big_s::S;

    #[test]
    fn normal() {
      let give = "shellcheck 0.9.0";
      let have = parse_line(give, 1, &apps::all()).unwrap();
      let want = Some(AppVersions {
        app_name: AppName::from("shellcheck"),
        versions: RequestedVersions::new(vec![RequestedVersion::Yard("0.9.0".into())]),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn multiple_versions() {
      let give = "shellcheck 0.9.0 0.6.0";
      let have = parse_line(give, 1, &apps::all()).unwrap();
      let want = Some(AppVersions {
        app_name: AppName::from("shellcheck"),
        versions: RequestedVersions::new(vec![RequestedVersion::Yard("0.9.0".into()), RequestedVersion::Yard("0.6.0".into())]),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn normal_with_multiple_spaces() {
      let give = "     shellcheck            0.9.0      ";
      let have = parse_line(give, 1, &apps::all()).unwrap();
      let want = Some(AppVersions {
        app_name: AppName::from("shellcheck"),
        versions: RequestedVersions::new(vec![RequestedVersion::Yard("0.9.0".into())]),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn normal_with_tabs() {
      let give = "shellcheck\t0.9.0";
      let have = parse_line(give, 1, &apps::all()).unwrap();
      let want = Some(AppVersions {
        app_name: AppName::from("shellcheck"),
        versions: RequestedVersions::new(vec![RequestedVersion::Yard("0.9.0".into())]),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn missing_version() {
      let give = "shellcheck ";
      let have = parse_line(give, 1, &apps::all());
      let want = Err(UserError::InvalidConfigFileFormat {
        line_no: 1,
        text: S("shellcheck"),
      });
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn empty_line() {
      let give = "";
      let have = parse_line(give, 1, &apps::all()).unwrap();
      assert_eq!(have, None);
    }

    #[test]
    fn spaces_only() {
      let give = "              ";
      let have = parse_line(give, 1, &apps::all()).unwrap();
      assert_eq!(have, None);
    }

    #[test]
    fn completely_commented_out() {
      let give = "# shellcheck 0.9.0";
      let have = parse_line(give, 1, &apps::all()).unwrap();
      assert_eq!(have, None);
    }

    #[test]
    fn valid_with_comment_at_end() {
      let give = "shellcheck 0.9.0  # comment";
      let have = parse_line(give, 1, &apps::all()).unwrap();
      let want = Some(AppVersions {
        app_name: AppName::from("shellcheck"),
        versions: RequestedVersions::new(vec![RequestedVersion::Yard("0.9.0".into())]),
      });
      pretty::assert_eq!(have, want);
    }
  }
}
