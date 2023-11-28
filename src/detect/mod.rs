mod cpu;
mod os;
mod platform;

use crate::Output;
use crate::Result;
pub use cpu::Cpu;
pub use os::Os;
pub use platform::Platform;

/// detects the platform this binary is running on
pub fn platform(output: &dyn Output) -> Result<Platform> {
    Ok(Platform {
        os: os::detect(output)?,
        cpu: cpu::determine(output)?,
    })
}
