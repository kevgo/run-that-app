use super::{Config, FILE_NAME};
use crate::error::UserError;
use crate::Result;
use std::fs::OpenOptions;
use std::io::Write;

pub fn save(config: &Config) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_NAME)
        .map_err(|err| UserError::CannotAccessConfigFile(err.to_string()))?;
    file.write_all(config.to_string().as_bytes())
        .map_err(|err| UserError::CannotAccessConfigFile(err.to_string()))?;
    Ok(())
}
