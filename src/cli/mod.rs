//! CLI arguments

mod app_version;
mod command;
mod exit;

#[cfg(test)]
use app_version::AppVersion;
pub(crate) use command::{parse, Command};
pub(crate) use exit::exit;
