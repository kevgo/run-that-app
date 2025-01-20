use crate::logging::{Event, Log};
use crate::run::{ExecutableFilename, ExecutablePath};
use which::which_global;

pub fn find_global_install(binary_name: ExecutableFilename, log: Log) -> Option<ExecutablePath> {
  log(Event::GlobalInstallSearch { binary: &binary_name });
  if let Ok(path) = which_global(binary_name.as_ref()) {
    log(Event::GlobalInstallFound { path: &path });
    Some(ExecutablePath(path))
  } else {
    log(Event::GlobalInstallNotFound);
    None
  }
}
