//! CLI arguments

mod app_version;
mod arguments;
mod command;
mod exit;

use app_version::AppVersion;
pub(crate) use arguments::parse;
pub(crate) use command::Command;
pub(crate) use exit::exit;
