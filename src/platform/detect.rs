use super::cpu;
use super::os;
use super::Platform;
use crate::Output;
use crate::Result;

/// detects the platform this binary is running on
pub fn detect(output: &dyn Output) -> Result<Platform> {
  Ok(Platform {
    os: os::detect(output)?,
    cpu: cpu::determine(output)?,
  })
}
