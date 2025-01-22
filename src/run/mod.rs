//! all the logic around running applications

mod check_output;
mod executable_call;
mod executable_name_platform;
mod executable_name_unix;
mod executable_path;
mod exit_status_to_code;
mod format_call;
mod method;
mod stream_output;

pub use check_output::check_output;
pub use executable_call::ExecutableCall;
pub use executable_name_platform::ExecutableNamePlatform;
pub use executable_name_unix::ExecutableNameUnix;
pub use executable_path::ExecutablePath;
pub use exit_status_to_code::exit_status_to_code;
pub use format_call::format_call;
pub use method::{Method, OtherAppArgs};
pub use stream_output::stream_output;
