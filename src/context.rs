//! Runtime context for passing common parameters through function calls.

use crate::configuration;
use crate::logging::Log;
use crate::platform::Platform;
use crate::yard::Yard;

/// Context struct that holds common runtime parameters to avoid passing
/// many parameters through multiple layers of function calls.
#[derive(Clone, Copy)]
pub(crate) struct RuntimeContext<'a> {
  pub(crate) platform: Platform,
  pub(crate) yard: &'a Yard,
  pub(crate) config_file: &'a configuration::File,
  pub(crate) log: Log,
}
