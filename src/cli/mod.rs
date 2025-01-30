//! CLI arguments

mod app_version;
mod command;
mod exit;

use app_version::AppVersion;
#[cfg(test)]
pub(crate) use command::{parse, Command};
pub(crate) use exit::exit;
