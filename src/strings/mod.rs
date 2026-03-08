//! This module implements functionality around regular expressions.

mod first_capture;
mod strip_prefix;

pub(crate) use first_capture::first_capture;
pub(crate) use strip_prefix::strip_prefix;
