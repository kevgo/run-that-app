//! This module implements logic to run the various forms of executables that applications can have.

mod executable;
mod executable_call;
mod executable_name;
mod method;

pub(crate) use executable::Executable;
pub(crate) use executable_call::{ExecutableArgs, ExecutableCall, ExecutableCallDefinition};
pub(crate) use executable_name::{ExecutableNamePlatform, ExecutableNameUnix};
pub(crate) use method::Method;
