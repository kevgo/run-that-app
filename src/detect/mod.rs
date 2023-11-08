mod cpu;
mod os;

use crate::cli::Output;
use crate::Result;
pub use cpu::Cpu;
pub use os::Os;
use std::fmt::Display;

pub fn detect(output: &Output) -> Result<Platform> {
    Ok(Platform {
        os: os::detect(output)?,
        cpu: cpu::determine(output)?,
    })
}

/// description of the local platform that the binary must be able to execute on
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Platform {
    pub os: Os,
    pub cpu: Cpu,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.os, self.cpu))
    }
}
