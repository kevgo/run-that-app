//! Runs applications.

mod executable;
mod execute_check_output;
mod execute_stream_output;
mod exit_status_to_code;
mod format_call;

pub use executable::Executable;
pub use execute_check_output::execute_check_output;
pub use execute_stream_output::execute_stream_output;
pub use exit_status_to_code::exit_status_to_code;
pub use format_call::format_call;
