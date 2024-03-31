use super::{root_folder, Yard};
use crate::prelude::*;
use std::fs;
use std::path::Path;

pub fn create(containing_folder: &Path) -> Result<Yard> {
    let root = root_folder(containing_folder);
    if let Err(err) = fs::create_dir_all(&root) {
        return Err(UserError::CannotCreateFolder {
            folder: root,
            reason: err.to_string(),
        });
    }
    Ok(Yard { root })
}
