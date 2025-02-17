//! all the logic around running applications

mod executable_args;
mod executable_call;
mod executable_name_platform;
mod executable_name_unix;
mod executable_path;
mod method;

pub(crate) use executable_args::ExecutableArgs;
pub(crate) use executable_call::{ExecutableCall, ExecutableCallDefinition};
pub(crate) use executable_name_platform::ExecutableNamePlatform;
pub(crate) use executable_name_unix::ExecutableNameUnix;
pub(crate) use executable_path::ExecutablePath;
pub(crate) use method::Method;
