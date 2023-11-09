//! The area on disk that stores the installed applications.
//! Named after rail yards, i.e. locations where passenger cars of trains are stored, sorted, and repaired.

mod load;
mod runnable_app;
mod yard;

pub use load::load;
pub use runnable_app::RunnableApp;
use std::path::PathBuf;
pub use yard::Yard;

/// provides the location of the production yard
pub fn production_location() -> PathBuf {
    todo!()
}
