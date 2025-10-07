use crate::error::{Result, UserError};
use std::io::ErrorKind;
use std::{env, fs};

/// looks for the file with the given name in the current or parent folders, and provides its content if it finds one
pub(crate) fn read_file(name: &str) -> Result<Option<String>> {
  let cwd = env::current_dir().map_err(|err| UserError::CannotDetermineCurrentDirectory(err.to_string()))?;
  let mut dir = cwd.as_path();
  loop {
    let file_path = dir.join(name);
    match fs::read_to_string(file_path) {
      Ok(text) => return Ok(Some(text)),
      Err(err) => match err.kind() {
        ErrorKind::NotFound => {
          // config file not found --> look in the parent folder
          dir = match dir.parent() {
            Some(parent) => parent,
            None => return Ok(None),
          };
        }
        ErrorKind::IsADirectory => {
          // we have reached the ".run-that-app" folder in the home directory --> give up looking
          return Ok(None);
        }
        _ => return Err(UserError::CannotAccessConfigFile(err.to_string())),
      },
    }
  }
}
