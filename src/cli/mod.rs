//! CLI arguments

mod args;
mod command;
mod requested_app;

pub use args::parse;
#[cfg(test)]
pub use args::Args;
pub use command::Command;
pub use requested_app::RequestedApp;
