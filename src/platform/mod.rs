//! This module implements detection of the local hardware and software.

mod cpu;
mod detect;
mod os;
#[allow(clippy::module_inception)]
mod platform;

pub(crate) use cpu::Cpu;
pub(crate) use detect::detect;
pub(crate) use os::Os;
pub(crate) use platform::Platform;
