//! This module implements a flexible logging mechanism.

mod event;
#[macro_use]
mod eprintf;
mod log;
mod normal;
mod verbose;

pub(crate) use event::Event;
pub(crate) use log::{new, Log};
