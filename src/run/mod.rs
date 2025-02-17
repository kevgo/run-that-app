//! all the logic around running applications

mod check_output;
mod executable_args;
mod executable_call;
mod executable_name_platform;
mod executable_name_unix;
mod executable_path;
mod exit_status_to_code;
mod method;
mod stream_output;

pub(crate) use check_output::check_output;
pub(crate) use executable_args::ExecutableArgs;
pub(crate) use executable_call::{ExecutableCall, ExecutableCallDefinition};
pub(crate) use executable_name_platform::ExecutableNamePlatform;
pub(crate) use executable_name_unix::ExecutableNameUnix;
pub(crate) use executable_path::ExecutablePath;
pub(crate) use exit_status_to_code::exit_status_to_code;
pub(crate) use method::Method;
pub(crate) use stream_output::stream_output;
