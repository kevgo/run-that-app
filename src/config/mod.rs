//! functionality for the `.tool-versions` file

mod config;
mod load;

pub use config::Config;
pub use load::{load, FILE_NAME};
