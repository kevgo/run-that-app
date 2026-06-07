//! This module encapsulates interaction with the CLI: parsing arguments and exiting with a status code.

mod app_version;
mod command;
mod exit;
mod parse;

use app_version::AppVersion;
pub use command::Command;
pub use exit::exit;
pub use parse::parse;
