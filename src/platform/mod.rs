mod cpu;
mod detect;
mod os;
#[allow(clippy::module_inception)]
mod platform;

pub use cpu::Cpu;
pub use detect::detect;
pub use os::Os;
pub use platform::Platform;
