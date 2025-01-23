//! The area on disk that stores the installed applications.
//! Named after rail yards, i.e. locations where passenger cars of trains are stored, sorted, and repaired.

mod app_folder;
#[allow(clippy::module_inception)] // I can't come up with a better name for this
mod yard;

use crate::prelude::*;
use std::path::{Path, PathBuf};
pub use yard::Yard;

/// provides the location of the production yard
pub fn production_location() -> Result<PathBuf> {
  let Some(home_dir) = dirs::home_dir() else {
    return Err(UserError::CannotDetermineHomeDirectory);
  };
  Ok(home_dir)
}

pub fn root_path(containing_folder: &Path) -> PathBuf {
  containing_folder.join(".run-that-app")
}
