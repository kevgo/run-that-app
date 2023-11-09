use super::{root_folder, Yard};
use crate::error::UserError;
use crate::Result;
use std::path::Path;

pub fn load(containing_folder: &Path) -> Result<Option<Yard>> {
    let root = root_folder(containing_folder);
    let Ok(metadata) = root.metadata() else {
        return Ok(None);
    };
    if !metadata.is_dir() {
        return Err(UserError::YardRootIsNotFolder { root });
    }
    Ok(Some(Yard { root }))
}
