//! Runs applications.

mod execute;
mod exit_status_to_code;
mod stream;

pub use execute::{query, run};
pub use exit_status_to_code::exit_status_to_code;
pub use stream::stream;
