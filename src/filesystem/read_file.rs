use std::{env, fs, io};

use crate::error::UserError;
/// provides the textual content of the config file
use crate::Result;

pub fn read_file(name: &str) -> Result<Option<String>> {
    let cwd = env::current_dir().map_err(|err| UserError::CannotDetermineCurrentDirectory(err.to_string()))?;
    let mut dir = cwd.as_path();
    loop {
        let file_path = dir.join(name);
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
