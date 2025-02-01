//! CLI arguments

mod app_version;
mod arguments;
mod command;
mod exit;

use app_version::AppVersion;
pub(crate) use arguments::parse;
#[cfg(test)]
pub(crate) use arguments::Arguments;
pub(crate) use command::Command;
pub(crate) use exit::exit;
