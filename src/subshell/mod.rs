//! Runs applications.

mod call_signature;
mod executable;
mod execute;
mod exit_status_to_code;
mod stream;

pub use call_signature::call_signature;
pub use executable::Executable;
pub use execute::run;
pub use exit_status_to_code::exit_status_to_code;
pub use stream::stream;
