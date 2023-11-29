use std::str::SplitAsciiWhitespace;

use crate::cli::RequestedApp;
use crate::{Result, UserError};

pub fn parse_line(line_text: &str, line_no: usize, acc: &mut Vec<RequestedApp>) -> Result<()> {
    let line_text = line_text.trim();
    let mut parts = LinePartsIterator::from(line_text);
    let Some(name) = parts.next() else {
        return Ok(());
    };
    let Some(version) = parts.next() else {
        return Err(UserError::InvalidConfigFileFormat {
            line_no,
            text: line_text.to_string(),
        });
    };
    if parts.next().is_some() {
        return Err(UserError::InvalidConfigFileFormat {
            line_no,
            text: line_text.to_string(),
        });
    }
    acc.push(RequestedApp {
        name: name.to_string(),
        version: version.to_string(),
    });
    Ok(())
}

/// provides non-whitespace and non-comment elements of the given line
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
        while let Some(part) = self.parts.next() {
            if part.starts_with('#') {
                return None;
            }
            return Some(part);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    mod parse_line {
        use super::super::parse_line;
        use crate::cli::RequestedApp;
        use crate::error::UserError;
        use big_s::S;

        #[test]
        fn normal() {
            let give = "shellcheck 0.9.0";
            let mut have = vec![];
            parse_line(give, 1, &mut have).unwrap();
            let want = vec![RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            }];
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn normal_with_multiple_spaces() {
            let give = "     shellcheck            0.9.0      ";
            let mut have = vec![];
            parse_line(give, 1, &mut have).unwrap();
            let want = vec![RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            }];
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn normal_with_tabs_spaces() {
            let give = "shellcheck\t0.9.0";
            let mut have = vec![];
            parse_line(give, 1, &mut have).unwrap();
            let want = vec![RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            }];
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn missing_version() {
            let give = "shellcheck ";
            let mut acc = vec![];
            let result = parse_line(give, 1, &mut acc);
            let want = Err(UserError::InvalidConfigFileFormat {
                line_no: 1,
                text: S("shellcheck"),
            });
            pretty::assert_eq!(result, want);
            assert_eq!(acc, vec![]);
        }

        #[test]
        fn empty_line() {
            let give = "";
            let mut acc = vec![];
            parse_line(give, 1, &mut acc).unwrap();
            assert_eq!(acc, vec![]);
        }

        #[test]
        fn spaces_only() {
            let give = "              ";
            let mut acc = vec![];
            parse_line(give, 1, &mut acc).unwrap();
            assert_eq!(acc, vec![]);
        }

        #[test]
        fn commented_out() {
            let give = "# shellcheck 0.9.0";
            let mut acc = vec![];
            parse_line(give, 1, &mut acc).unwrap();
            assert_eq!(acc, vec![]);
        }
    }
}
