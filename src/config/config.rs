use super::{AppName, AppVersions, RequestedVersion, RequestedVersions, FILE_NAME};
use crate::apps::Apps;
use crate::error::UserError;
use crate::{filesystem, Result};
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::str::SplitAsciiWhitespace;

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    pub apps: Vec<AppVersions>,
}

impl Config {
    pub fn create() -> Result<()> {
        let mut file = match OpenOptions::new().write(true).create_new(true).open(FILE_NAME) {
            Ok(file) => file,
            Err(err) => {
                if err.kind() == ErrorKind::AlreadyExists {
                    return Err(UserError::ConfigFileAlreadyExists);
                }
                panic!("{}", err);
            }
        };
        let content = "\
# actionlint 1.2.26
# gh 2.39.1
";
        file.write_all(content.as_bytes()).map_err(|err| UserError::CannotAccessConfigFile(err.to_string()))
    }

    pub fn load(apps: &Apps) -> Result<Config> {
        match filesystem::read_file(FILE_NAME)? {
            Some(text) => parse(&text, apps),
            None => Ok(Config::default()),
        }
    }

    pub fn lookup(self, app_name: &AppName) -> Option<RequestedVersions> {
        self.apps.into_iter().find(|app| app.app == app_name).map(|app_version| app_version.versions)
    }

    pub fn save(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(FILE_NAME)
            .map_err(|err| UserError::CannotAccessConfigFile(err.to_string()))?;
        file.write_all(self.to_string().as_bytes())
            .map_err(|err| UserError::CannotAccessConfigFile(err.to_string()))?;
        Ok(())
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for AppVersions { app, versions } in &self.apps {
            f.write_str(app.as_str())?;
            f.write_str(" ")?;
            f.write_str(&versions.join(", "))?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn parse(text: &str, all_apps: &Apps) -> Result<Config> {
    let mut apps = vec![];
    for (i, line) in text.lines().enumerate() {
        if let Some(app_version) = parse_line(line, i, all_apps)? {
            apps.push(app_version);
        }
    }
    Ok(Config { apps })
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
    let mut versions = RequestedVersions(vec![RequestedVersion::parse(version, app)?]);
    for part in parts {
        versions.push(RequestedVersion::parse(part, app)?);
    }
    Ok(Some(AppVersions { app: name.into(), versions }))
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
        let Some(part) = self.parts.next() else {
            return None;
        };
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
        use crate::apps::Apps;
        use crate::config::{AppName, AppVersions, Config, RequestedVersions};

        #[test]
        fn normal() {
            let give = "alpha 1.2.3\n\
                        beta  2.3.4 # comment\n\
                        gamma 3.4.5 6.7.8\n\
                        delta system@3.4 5.6.7";
            let have = parse(give, &Apps::default()).unwrap();
            let want = Config {
                apps: vec![
                    AppVersions {
                        app: AppName::from("alpha"),
                        versions: RequestedVersions::parse(vec!["1.2.3"], &Apps::default()).unwrap(),
                    },
                    AppVersions {
                        app: AppName::from("beta"),
                        versions: RequestedVersions::parse(vec!["2.3.4"], &Apps::default()).unwrap(),
                    },
                    AppVersions {
                        app: AppName::from("gamma"),
                        versions: RequestedVersions::parse(vec!["3.4.5", "6.7.8"], &Apps::default()).unwrap(),
                    },
                    AppVersions {
                        app: AppName::from("delta"),
                        versions: RequestedVersions::parse(vec!["system@3.4", "5.6.7"], &Apps::default()).unwrap(),
                    },
                ],
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn empty() {
            let give = "";
            let have = parse(give, &Apps::default()).unwrap();
            let want = Config { apps: vec![] };
            pretty::assert_eq!(have, want);
        }
    }

    mod parse_line {
        use super::super::parse_line;
        use crate::apps::{Apps, ShellCheck};
        use crate::config::{AppName, AppVersions, RequestedVersion, RequestedVersions, Version};
        use crate::error::UserError;
        use big_s::S;

        #[test]
        fn normal() {
            let give = "shellcheck 0.9.0";
            let have = parse_line(give, 1, &Apps::default()).unwrap();
            let want = Some(AppVersions {
                app: AppName::from("shellcheck"),
                versions: RequestedVersions::parse(vec!["0.9.0"], &Apps::default()).unwrap(),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn multiple_versions() {
            let give = "shellcheck 0.9.0 0.6.0";
            let have = parse_line(give, 1, &Apps::default()).unwrap();
            let want = Some(AppVersions {
                app: AppName::from("shellcheck"),
                versions: RequestedVersions::parse(vec!["0.9.0", "0.6.0"], &Apps::default()).unwrap(),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn normal_with_multiple_spaces() {
            let give = "     shellcheck            0.9.0      ";
            let have = parse_line(give, 1, &Apps::default()).unwrap();
            let want = Some(AppVersions {
                app: AppName::from("shellcheck"),
                versions: RequestedVersions::parse(vec!["0.9.0"], &Apps::default()).unwrap(),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn normal_with_tabs() {
            let give = "shellcheck\t0.9.0";
            let have = parse_line(give, 1, &Apps(vec![Box::new(ShellCheck {})])).unwrap();
            let want = Some(AppVersions {
                app: AppName::from("shellcheck"),
                versions: RequestedVersions(vec![RequestedVersion::Yard(Version(S("0.9.0")))]),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_version() {
            let give = "shellcheck ";
            let have = parse_line(give, 1, &Apps::default());
            let want = Err(UserError::InvalidConfigFileFormat {
                line_no: 1,
                text: S("shellcheck"),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn empty_line() {
            let give = "";
            let have = parse_line(give, 1, &Apps::default()).unwrap();
            assert_eq!(have, None);
        }

        #[test]
        fn spaces_only() {
            let give = "              ";
            let have = parse_line(give, 1, &Apps::default()).unwrap();
            assert_eq!(have, None);
        }

        #[test]
        fn completely_commented_out() {
            let give = "# shellcheck 0.9.0";
            let have = parse_line(give, 1, &Apps::default()).unwrap();
            assert_eq!(have, None);
        }

        #[test]
        fn valid_with_comment_at_end() {
            let give = "shellcheck 0.9.0  # comment";
            let have = parse_line(give, 1, &Apps::default()).unwrap();
            let want = Some(AppVersions {
                app: AppName::from("shellcheck"),
                versions: RequestedVersions::parse(vec!["0.9.0"], &Apps::default()).unwrap(),
            });
            pretty::assert_eq!(have, want);
        }
    }
}
