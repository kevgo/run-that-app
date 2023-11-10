//! The area on disk that stores the installed applications.
//! Named after rail yards, i.e. locations where passenger cars of trains are stored, sorted, and repaired.

mod create;
mod load;
mod load_or_create;
mod runnable_app;
mod yard;

use crate::Result;
pub use create::create;
pub use load::load;
pub use load_or_create::load_or_create;
pub use runnable_app::RunnableApp;
use std::path::{Path, PathBuf};
pub use yard::Yard;

use crate::error::UserError;

/// provides the location of the production yard
pub fn production_location() -> Result<PathBuf> {
    let Some(home_dir) = dirs::home_dir() else {
        return Err(UserError::CannotDetermineHomeDirectory);
    };
    Ok(home_dir)
}

pub fn root_folder(containing_folder: &Path) -> PathBuf {
    containing_folder.join(".run-that-app")
}
