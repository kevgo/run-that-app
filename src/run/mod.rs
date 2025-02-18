//! all the logic around running applications

mod executable;
mod executable_args;
mod executable_call;
mod executable_name_platform;
mod executable_name_unix;
mod method;

pub(crate) use executable::Executable;
pub(crate) use executable_args::ExecutableArgs;
pub(crate) use executable_call::{ExecutableCall, ExecutableCallDefinition};
pub(crate) use executable_name_platform::ExecutableNamePlatform;
pub(crate) use executable_name_unix::ExecutableNameUnix;
pub(crate) use method::Method;
