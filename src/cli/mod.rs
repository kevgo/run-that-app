//! This module encapsulates interaction with the CLI: parsing arguments and exiting with a status code.

mod app_version;
mod command;
mod exit;
mod parse;

pub use app_version::AppVersion;
pub use command::Cli;
pub use exit::exit;
pub use parse::parse;
