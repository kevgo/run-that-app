use crate::Result;
use crate::UserError;
use std::path::PathBuf;
use std::{fs, io};

use crate::cli::RequestedApp;

pub fn load() -> Result<Vec<RequestedApp>> {
    let path = PathBuf::from(FILE_NAME);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => return Ok(vec![]),
            _ => return Err(UserError::CannotAccessConfigFile(err.to_string())),
        },
    };
    parse(text)
}

fn parse(text: String) -> Result<Vec<RequestedApp>> {
    let mut result = vec![];
    for (i, line) in text.lines().enumerate() {
        parse_line(line, i, &mut result)?;
    }
    Ok(result)
}

fn parse_line(line_text: &str, line_no: usize, acc: &mut Vec<RequestedApp>) -> Result<()> {
    let line_text = line_text.trim();
    if line_text.is_empty() {
        return Ok(());
    }
    let mut parts = line_text.split_ascii_whitespace();
    let app = parts.next() else {
        return Err(UserError::InvalidConfigFileFormat {
            line_no,
            text: line_text.to_string(),
        });
    };
    Ok(())
}

pub const FILE_NAME: &str = ".tools-versions";
