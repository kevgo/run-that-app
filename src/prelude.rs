//! stuff that is used throughout the crate

pub use crate::error::UserError;

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = core::result::Result<T, UserError>;
