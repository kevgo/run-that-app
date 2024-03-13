use super::normal;
use super::verbose;
use super::Event;

/// A function that logs the given event to the CLI.
/// There are several Log implementations that log at different verbosity levels.
pub type Log = fn(Event);

/// provides a logger function at the given verbosity level
pub fn new(verbose: bool) -> Log {
    if verbose {
        verbose::display
    } else {
        normal::display
    }
}
