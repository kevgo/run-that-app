use super::{AppName, Version};
use crate::error::UserError;
use crate::Result;

pub fn get_version(app: &AppName) -> Result<Version> {
    let config = super::load()?;
    match config.lookup(app) {
        Some(configured_version) => Ok(configured_version),
        None => return Err(UserError::RunRequestMissingVersion),
    }
}
