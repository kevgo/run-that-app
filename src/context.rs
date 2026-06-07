//! Runtime context for passing common parameters through function calls.

use crate::configuration;
use crate::logging::Log;
use crate::platform::Platform;
use crate::yard::Yard;

/// Context struct that holds common runtime parameters to avoid passing
/// many parameters through multiple layers of function calls.
#[derive(Clone, Copy)]
pub struct RuntimeContext<'a> {
  pub platform: Platform,
  pub yard: &'a Yard,
  pub config_file: &'a configuration::File,
  pub log: Log,
}
