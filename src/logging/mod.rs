//! This module implements a flexible logging mechanism.

mod event;
#[macro_use]
mod eprintf;
mod log;
mod normal;
mod verbose;

pub use event::Event;
pub use log::{Log, new};
