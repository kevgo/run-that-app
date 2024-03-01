//! CLI arguments

mod args;
mod command;

pub use args::parse;
#[cfg(test)]
pub use args::Args;
pub use command::Command;
