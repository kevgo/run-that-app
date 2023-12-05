//! functionality for the `.tool-versions` file

mod config;
mod load;
mod save;

pub use config::Config;
pub use load::{load, FILE_NAME};
pub use save::save;
