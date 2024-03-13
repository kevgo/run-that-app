use super::cpu;
use super::os;
use super::Platform;
use crate::Log;
use crate::Result;

/// detects the platform this binary is running on
pub fn detect(log: Log) -> Result<Platform> {
    Ok(Platform {
        os: os::detect(log)?,
        cpu: cpu::determine(log)?,
    })
}
