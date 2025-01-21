use crate::logging::{Event, Log};
use crate::run::{executable_name, ExecutablePath};
use which::which_global;

pub fn find_global_install(binary_name: &executable_name::Platform, log: Log) -> Option<ExecutablePath> {
  log(Event::GlobalInstallSearch { binary: binary_name });
  if let Ok(path) = which_global(binary_name.as_ref()) {
    log(Event::GlobalInstallFound { path: &path });
    Some(ExecutablePath::from(path))
  } else {
    log(Event::GlobalInstallNotFound);
    None
  }
}
