use super::{AppName, Config, Version, Versions};
use crate::error::UserError;
use crate::Result;

pub fn versions_for(app: &AppName, cli_version: Option<Version>) -> Result<Versions> {
    if let Some(version) = cli_version {
        return Ok(Versions::from(version));
    }
    let config = Config::load()?;
    let Some(versions) = config.lookup(app) else {
        return Err(UserError::RunRequestMissingVersion);
    };
    Ok(versions)
}
