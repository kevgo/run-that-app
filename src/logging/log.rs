use super::{Event, normal, verbose};

/// A function that logs the given event to the CLI.
///
/// You can get a logger by calling the [new] function.
pub type Log = fn(Event);

/// provides a logger function at the given verbosity level
#[must_use]
pub fn new(verbose: bool) -> Log {
  if verbose { verbose::log } else { normal::log }
}
