use super::{AppName, Config, Version, Versions};
use crate::error::UserError;
use crate::Result;

/// provides
pub fn versions_for(app: &AppName, cli_version: Option<Version>) -> Result<Versions> {
    if let Some(version) = cli_version {
        return Ok(Versions::from(version));
    }
    match Config::load()?.lookup(app) {
        Some(versions) => Ok(versions),
        None => Err(UserError::RunRequestMissingVersion),
    }
}
