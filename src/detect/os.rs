use crate::ui::Output;
use crate::{Result, UserError};
use std::env;
use std::fmt::Display;

pub fn detect(output: &dyn Output) -> Result<Os> {
    output.log(CATEGORY, &format!("OS id: {}", env::consts::OS));
    match env::consts::OS {
        "linux" => Ok(Os::Linux),
        "macos" => Ok(Os::MacOS),
        "windows" => Ok(Os::Windows),
        _ => Err(UserError::CannotDetermineOS),
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

const CATEGORY: &str = "detect/os";
