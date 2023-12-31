//! CLI arguments

mod args;
mod command;
mod requested_app;

pub use args::{parse, Args};
pub use command::Command;
pub use requested_app::RequestedApp;
