use super::{AppName, Config, Version, Versions};
use crate::error::UserError;
use crate::Result;

/// provides the version to use: if the user provided a version to use via CLI, use it.
/// Otherwise provide the versions from the config file.
pub fn versions_for(app: &AppName, cli_version: Option<Version>) -> Result<Versions> {
    if let Some(version) = cli_version {
        return Ok(Versions::from(version));
    }
    match Config::load()?.lookup(app) {
        Some(versions) => Ok(versions),
        None => Err(UserError::RunRequestMissingVersion),
    }
}
