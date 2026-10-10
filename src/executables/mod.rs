//! This module implements logic to run the various forms of executables that applications can have.

mod callable;
mod cmd_to_string;
mod executable;
mod executable_name;
mod load;
mod load_from_path;
mod load_from_yard;
mod load_or_install;
mod run_method;
mod uv_tool;

pub use callable::Callable;
pub use cmd_to_string::cmd_to_string;
pub use executable::Executable;
pub use executable_name::{ExecutableNamePlatform, ExecutableNameUnix};
pub use load::{LoadAppOutcome, load_app_versions};
pub use load_from_path::load_from_path;
pub use load_from_yard::load_from_yard;
pub use load_or_install::{LoadOrInstallAppAndCarrierArgs, LoadOrInstallAppOutcome, load_or_install_app_and_carrier, load_or_install_apps};
pub use run_method::RunMethod;
pub use uv_tool::UvTool;
