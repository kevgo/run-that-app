use super::{Platform, cpu, os};
use crate::Log;
use crate::error::Result;

/// detects the platform this binary is running on
pub(crate) fn detect(log: Log) -> Result<Platform> {
  Ok(Platform {
    os: os::detect(log)?,
    cpu: cpu::determine(log)?,
  })
}
