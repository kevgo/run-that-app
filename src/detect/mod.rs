mod cpu;
mod os;
mod platform;

use crate::Output;
use crate::Result;
pub use cpu::Cpu;
pub use os::Os;
pub use platform::Platform;

pub fn detect(output: &dyn Output) -> Result<Platform> {
    Ok(Platform {
        os: os::detect(output)?,
        cpu: cpu::determine(output)?,
    })
}
