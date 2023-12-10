use crate::{Result, UserError};
use std::fs;
use std::path::Path;

pub fn create_parent(filepath: &Path) -> Result<()> {
    if let Some(parent) = filepath.parent() {
        fs::create_dir_all(parent).map_err(|err| UserError::CannotCreateFolder {
            folder: parent.to_path_buf(),
            reason: err.to_string(),
        })?;
    }
    Ok(())
}
