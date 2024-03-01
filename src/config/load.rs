use super::Config;
use super::FILE_NAME;
use crate::config::AppVersion;
use crate::Result;
use crate::UserError;
use std::env;
use std::str::SplitAsciiWhitespace;
use std::{fs, io};

pub fn find() -> Result<Option<String>> {
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

pub fn load() -> Result<Config> {
    match find()? {
        Some(text) => parse(&text),
        None => Ok(Config::default()),
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

pub fn parse_line(line_text: &str, line_no: usize) -> Result<Option<AppVersion>> {
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
        name: name.to_string(),
        version: version.into(),
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
        use crate::config::{AppVersion,Config};
        use big_s::S;

        #[test]
        fn normal() {
            let give = "alpha 1.2.3\n\
                        beta  2.3.4 # comment\n\
                        gamma 3.4.5";
            let have = parse(give).unwrap();
            let want = Config {
                apps: vec![
                    AppVersion {
                        name: S("alpha"),
                        version: "1.2.3".into(),
                    },
                    AppVersion {
                        name: S("beta"),
                        version: "2.3.4".into(),
                    },
                    AppVersion {
                        name: S("gamma"),
                        version: "3.4.5".into(),
                    },
                ],
            };
            pretty::assert_eq!(have, want);
        }
    }

    mod parse_line {
        use super::super::parse_line;
        use crate::config::AppVersion;
        use crate::error::UserError;
        use big_s::S;

        #[test]
        fn normal() {
            let give = "shellcheck 0.9.0";
            let have = parse_line(give, 1).unwrap();
            let want = Some(AppVersion {
                name: S("shellcheck"),
                version: "0.9.0".into(),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn normal_with_multiple_spaces() {
            let give = "     shellcheck            0.9.0      ";
            let have = parse_line(give, 1).unwrap();
            let want = Some(AppVersion {
                name: S("shellcheck"),
                version: "0.9.0".into(),
            });
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn normal_with_tabs() {
            let give = "shellcheck\t0.9.0";
            let have = parse_line(give, 1).unwrap();
            let want = Some(AppVersion {
                name: S("shellcheck"),
                version: "0.9.0".into(),
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
                name: S("shellcheck"),
                version: "0.9.0".into(),
            });
            pretty::assert_eq!(have, want);
        }
    }
}
