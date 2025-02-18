//! This module implements logic around executables of applications.

mod executable_call;
mod executable_file;
mod executable_name;
mod method;

pub(crate) use executable_call::{ExecutableArgs, ExecutableCall, ExecutableCallDefinition};
pub(crate) use executable_file::ExecutableFile;
pub(crate) use executable_name::{ExecutableNamePlatform, ExecutableNameUnix};
pub(crate) use method::Method;
