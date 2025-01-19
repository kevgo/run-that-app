use crate::logging::{Event, Log};
use crate::run::ExecutablePath;
use which::which_global;

pub fn find_global_install(binary_name: &str, log: Log) -> Option<ExecutablePath> {
  log(Event::GlobalInstallSearch { binary: binary_name });
  if let Ok(path) = which_global(binary_name) {
    log(Event::GlobalInstallFound { path: &path });
    Some(ExecutablePath(path))
  } else {
    log(Event::GlobalInstallNotFound);
    None
  }
}
