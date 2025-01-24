//! Output to the user

mod event;
#[macro_use]
mod eprintf;
mod log;
mod normal;
mod verbose;

pub(crate) use event::Event;
pub(crate) use log::{new, Log};
