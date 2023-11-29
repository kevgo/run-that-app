use super::parse_line;
use super::Config;
use crate::Result;
use crate::UserError;
use std::{fs, io};

pub fn load() -> Result<Config> {
    let text = match fs::read_to_string(FILE_NAME) {
        Ok(text) => text,
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => return Ok(Config::default()),
            _ => return Err(UserError::CannotAccessConfigFile(err.to_string())),
        },
    };
    parse(&text)
}

fn parse(text: &str) -> Result<Config> {
    let mut result = vec![];
    for (i, line) in text.lines().enumerate() {
        parse_line(line, i, &mut result)?;
    }
    Ok(Config(result))
}

pub const FILE_NAME: &str = ".tools-versions";

#[cfg(test)]
mod tests {

    mod parse {
        use super::super::parse;
        use crate::cli::RequestedApp;
        use crate::config::Config;
        use big_s::S;

        #[test]
        fn normal() {
            let give = "alpha 1.2.3\n\
                        beta  2.3.4\n\
                        gamma 3.4.5";
            let have = parse(give).unwrap();
            let want = Config(vec![
                RequestedApp {
                    name: S("alpha"),
                    version: S("1.2.3"),
                },
                RequestedApp {
                    name: S("beta"),
                    version: S("2.3.4"),
                },
                RequestedApp {
                    name: S("gamma"),
                    version: S("3.4.5"),
                },
            ]);
            pretty::assert_eq!(have, want);
        }
    }
}
