use super::Event;
use super::{normal, verbose};

/// A function that logs the given event to the CLI.
/// There are several types of loggers at different verbosity levels.
pub type Log = fn(Event);

/// provides a logger function at the given verbosity level
pub fn new(verbose: bool) -> Log {
  if verbose {
    verbose::log
  } else {
    normal::log
  }
}
