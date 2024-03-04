use super::{AppName, AppVersion, Version, FILE_NAME};
use crate::error::UserError;
use crate::Result;
use std::fmt::Display;
use std::str::SplitAsciiWhitespace;
use std::{env, fs, io};

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    pub apps: Vec<AppVersion>,
}

impl Config {
    pub fn load() -> Result<Config> {
        match read()? {
            Some(text) => parse(&text),
            None => Ok(Config::default()),
        }
    }

    pub fn lookup(self, app_name: &AppName) -> Option<Version> {
        self.apps.into_iter().find(|app| app.app == app_name).map(|app_version| app_version.version)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for AppVersion { app, version } in &self.apps {
            f.write_str(app.as_str())?;
            f.write_str(" ")?;
            f.write_str(version.as_str())?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn parse(text: &str) -> Result<Config> {
    let mut apps = vec![];
    for (i, line) in text.lines().enumerate() {
        if let Some(app_version) = parse_line(line, i)? {
            apps.push(app_version);
        }
    }
    Ok(Config { apps })
}

fn parse_line(line_text: &str, line_no: usize) -> Result<Option<AppVersion>> {
    let line_text = line_text.trim();
    let mut parts = LinePartsIterator::from(line_text);
    let Some(name) = parts.next() else {
        // empty or commented out line --> ignore
        return Ok(None);
    };
    let Some(version) = parts.next() else {
        // line has only one element --> invalid
        return Err(UserError::InvalidConfigFileFormat {
            line_no,
            text: line_text.to_string(),
        });
    };
    if parts.next().is_some() {
        // line has more than 2 elements --> invalid
        return Err(UserError::InvalidConfigFileFormat {
            line_no,
            text: line_text.to_string(),
        });
    }
    Ok(Some(AppVersion {
        app: name.into(),
        version: version.into(),
    }))
}

/// provides the textual content of the config file
fn read() -> Result<Option<String>> {
    let cwd = env::current_dir().map_err(|err| UserError::CannotDetermineCurrentDirectory(err.to_string()))?;
    let mut dir = cwd.as_path();
    loop {
        let file_path = dir.join(FILE_NAME);
        match fs::read_to_string(file_path) {
            Ok(text) => return Ok(Some(text)),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => {}
                _ => return Err(UserError::CannotAccessConfigFile(err.to_string())),
            },
        }
        dir = match dir.parent() {
            Some(parent) => parent,
            None => return Ok(None),
        };
    }
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
        use crate::config::{AppName, AppVersion, Config, Version};

        #[test]
        fn normal() {
            let give = "alpha 1.2.3\n\
                        beta  2.3.4 # comment\n\
                        gamma 3.4.5";
            let have = parse(give).unwrap();
            let want = Config {
                apps: vec![
                    AppVersion {
                        app: AppName::from("alpha"),
                        version: Version::from("1.2.3"),
                    },
                    AppVersion {
                        app: AppName::from("beta"),
                        version: Version::from("2.3.4"),
                    },
                    AppVersion {
                        app: AppName::from("gamma"),
                        version: Version::from("3.4.5"),
                    },
                ],
            };
            pretty::assert_eq!(have, want);
        }
    }

    mod parse_line {
        use super::super::parse_line;
        use crate::config::{AppName, AppVersion, Version};
        use crate::error::UserError;
        use big_s::S;

        #[test]
        fn normal() {
            let give = "shellcheck 0.9.0";
            let have = parse_line(give, 1).unwrap();
            let want = Some(AppVersion {
                app: AppName::from("shellcheck"),
                version: Version::from("0.9.0"),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn normal_with_multiple_spaces() {
            let give = "     shellcheck            0.9.0      ";
            let have = parse_line(give, 1).unwrap();
            let want = Some(AppVersion {
                app: AppName::from("shellcheck"),
                version: Version::from("0.9.0"),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn normal_with_tabs() {
            let give = "shellcheck\t0.9.0";
            let have = parse_line(give, 1).unwrap();
            let want = Some(AppVersion {
                app: AppName::from("shellcheck"),
                version: Version::from("0.9.0"),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_version() {
            let give = "shellcheck ";
            let have = parse_line(give, 1);
            let want = Err(UserError::InvalidConfigFileFormat {
                line_no: 1,
                text: S("shellcheck"),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn empty_line() {
            let give = "";
            let have = parse_line(give, 1).unwrap();
            assert_eq!(have, None);
        }

        #[test]
        fn spaces_only() {
            let give = "              ";
            let have = parse_line(give, 1).unwrap();
            assert_eq!(have, None);
        }

        #[test]
        fn completely_commented_out() {
            let give = "# shellcheck 0.9.0";
            let have = parse_line(give, 1).unwrap();
            assert_eq!(have, None);
        }

        #[test]
        fn valid_with_comment_at_end() {
            let give = "shellcheck 0.9.0  # comment";
            let have = parse_line(give, 1).unwrap();
            let want = Some(AppVersion {
                app: AppName::from("shellcheck"),
                version: Version::from("0.9.0"),
            });
            pretty::assert_eq!(have, want);
        }
    }
}
