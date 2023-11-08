use crate::cli::Output;
use crate::{Result, UserError};
use std::env;
use std::fmt::Display;

pub fn determine(output: &Output) -> Result<Cpu> {
    output.log(CATEGORY, &format!("CPU id: {}", env::consts::ARCH));
    match env::consts::ARCH {
        "x86_64" => Ok(Cpu::Intel64),
        "aarch64" => Ok(Cpu::Arm64),
        _ => Err(UserError::CannotDetermineCPU),
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

const CATEGORY: &str = "detect/cpu";
