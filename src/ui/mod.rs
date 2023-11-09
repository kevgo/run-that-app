//! The outer layer interfacing with the user: CLI arguments, printing to the console

pub mod cli_args;
pub mod output;

pub use cli_args::{Args, Command, RequestedApp};
pub use output::{ConsoleOutput, Output};
