use super::parse_line;
use super::Config;
use crate::Result;
use crate::UserError;
use std::path::PathBuf;
use std::{fs, io};

pub fn load() -> Result<Config> {
    let path = PathBuf::from(FILE_NAME);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => return Ok(Config::default()),
            _ => return Err(UserError::CannotAccessConfigFile(err.to_string())),
        },
    };
    parse(text)
}

fn parse(text: String) -> Result<Config> {
    let mut result = vec![];
    for (i, line) in text.lines().enumerate() {
        parse_line(line, i, &mut result)?;
    }
    Ok(Config(result))
}

pub const FILE_NAME: &str = ".tools-versions";
