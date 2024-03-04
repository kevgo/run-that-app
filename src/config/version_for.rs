use super::{AppName, Config, Version};
use crate::error::UserError;
use crate::Result;

pub fn version_for(app: &AppName, cli_version: Option<Version>) -> Result<Version> {
    if let Some(version) = cli_version {
        return Ok(version);
    }
    let config = Config::load()?;
    match config.lookup(app) {
        Some(version) => Ok(version),
        None => Err(UserError::RunRequestMissingVersion),
    }
}
