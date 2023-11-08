mod cpu;
mod os;
mod platform;

use crate::cli::Output;
use crate::Result;
pub use cpu::Cpu;
pub use os::Os;
pub use platform::Platform;

pub fn detect(output: &Output) -> Result<Platform> {
    Ok(Platform {
        os: os::detect(output)?,
        cpu: cpu::determine(output)?,
    })
}
