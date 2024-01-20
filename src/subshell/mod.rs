//! Runs applications.

mod executable;
mod execute;
mod exit_status_to_code;
mod stream;

pub use executable::Executable;
pub use execute::run;
pub use exit_status_to_code::exit_status_to_code;
pub use stream::stream;
