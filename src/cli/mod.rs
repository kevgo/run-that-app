//! CLI arguments

mod app_version;
mod args;
mod command;

pub use app_version::AppVersion;
pub use args::parse;
#[cfg(test)]
pub use args::Args;
pub use command::Command;
