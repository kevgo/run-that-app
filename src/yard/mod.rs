//! This module manages the locally installed applications.

#[allow(clippy::module_inception)] // I can't come up with a better name for this
mod yard;

use crate::error::{Result, UserError};
use std::path::{Path, PathBuf};
pub(crate) use yard::Yard;

/// provides the location of the production yard
pub(crate) fn production_location() -> Result<PathBuf> {
  let Some(home_dir) = dirs::home_dir() else {
    return Err(UserError::CannotDetermineHomeDirectory);
  };
  Ok(home_dir)
}

pub(crate) fn root_path(containing_folder: &Path) -> PathBuf {
  containing_folder.join(".run-that-app")
}
