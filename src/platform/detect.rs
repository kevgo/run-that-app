use super::cpu;
use super::os;
use super::Platform;
use crate::LogFn;
use crate::Result;

/// detects the platform this binary is running on
pub fn detect(log: LogFn) -> Result<Platform> {
    Ok(Platform {
        os: os::detect(log)?,
        cpu: cpu::determine(log)?,
    })
}
