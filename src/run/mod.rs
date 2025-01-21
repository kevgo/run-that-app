//! all the logic around running applications

mod check_output;
mod executable_call;
pub mod executable_name;
mod executable_path;
mod exit_status_to_code;
mod format_call;
mod method;
mod stream_output;

pub use check_output::check_output;
pub use executable_call::ExecutableCall;
pub use executable_path::ExecutablePath;
pub use exit_status_to_code::exit_status_to_code;
pub use format_call::format_call;
pub use method::Method;
pub use stream_output::stream_output;
