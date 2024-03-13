use crate::output::{Event, Log};
use crate::{Result, UserError};
use std::env;
use std::fmt::Display;

pub fn detect(log: Log) -> Result<Os> {
    log(Event::IdentifiedOs { name: env::consts::OS });
    match env::consts::OS {
        "linux" => Ok(Os::Linux),
        "macos" => Ok(Os::MacOS),
        "windows" => Ok(Os::Windows),
        other => Err(UserError::UnsupportedOS(other.to_string())),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Os {
    Windows,
    Linux,
    MacOS,
}

impl Display for Os {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Os::Windows => "windows",
            Os::Linux => "linux",
            Os::MacOS => "macOS",
        };
        f.write_str(text)
    }
}
