//! This module encapsulates interaction with the CLI: parsing arguments and exiting with a status code.

mod app_version;
mod command;
mod exit;
mod parse;

pub(crate) use app_version::AppVersion;
pub(crate) use command::Command;
pub(crate) use exit::exit;
pub(crate) use parse::parse;
