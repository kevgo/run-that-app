//! The outer layer interfacing with the user: CLI arguments, printing to the console

mod cli_args;
pub mod output;

pub use cli_args::{parse, Args, Command, RequestedApp};
pub use output::Output;
