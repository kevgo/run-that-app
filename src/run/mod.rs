//! all the logic around running applications

mod capture_output;
mod executable_args;
mod executable_call;
mod executable_name_platform;
mod executable_name_unix;
mod executable_path;
mod exit_status_to_code;
mod method;

use capture_output::capture_output;
pub(crate) use executable_args::ExecutableArgs;
pub(crate) use executable_call::{add_paths, ExecutableCall, ExecutableCallDefinition};
pub(crate) use executable_name_platform::ExecutableNamePlatform;
pub(crate) use executable_name_unix::ExecutableNameUnix;
pub(crate) use executable_path::ExecutablePath;
pub(crate) use exit_status_to_code::exit_status_to_code;
pub(crate) use method::Method;
