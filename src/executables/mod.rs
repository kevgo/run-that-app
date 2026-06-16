//! This module implements logic to run the various forms of executables that applications can have.

mod command_info;
mod executable;
mod executable_call;
mod executable_name;
mod run_method;

pub use command_info::CommandInfo;
pub use executable::Executable;
pub use executable_call::{ExecutableArgs, ExecutableCall, ExecutableCallDefinition};
pub use executable_name::{ExecutableNamePlatform, ExecutableNameUnix};
pub use run_method::RunMethod;
