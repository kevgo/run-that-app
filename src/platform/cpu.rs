use crate::logger::{Event, Log};
use crate::prelude::*;
use std::env;
use std::fmt::Display;

pub fn determine(log: Log) -> Result<Cpu> {
  log(Event::IdentifiedCpu { architecture: env::consts::ARCH });
  match env::consts::ARCH {
    "x86_64" => Ok(Cpu::Intel64),
    "aarch64" => Ok(Cpu::Arm64),
    other => Err(UserError::UnsupportedCPU(other.to_string())),
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cpu {
  Intel64,
  Arm64,
}

impl Display for Cpu {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let text = match self {
      Cpu::Intel64 => "intel64",
      Cpu::Arm64 => "arm64",
    };
    f.write_str(text)
  }
}
