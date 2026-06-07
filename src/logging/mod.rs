//! This module implements a flexible logging mechanism.

mod event;
#[macro_use]
mod eprintf;
mod log;
mod normal;
mod verbose;

pub use event::Event;
pub use log::{Log, new};
pub use normal::log as normal_log;
pub use verbose::log as verbose_log;
