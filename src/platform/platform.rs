use super::{Cpu, Os};
use std::fmt::Display;

/// description of the local platform that the binary must be able to execute on
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Platform {
    pub os: Os,
    pub cpu: Cpu,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{os}/{cpu}", os = self.os, cpu = self.cpu))
    }
}
