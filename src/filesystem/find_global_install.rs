use crate::run::Executable;
use crate::logging::{Event, Log};
use which::which_global;

pub fn find_global_install(binary_name: &str, log: Log) -> Option<Executable> {
  log(Event::GlobalInstallSearch { binary: binary_name });
  if let Ok(path) = which_global(binary_name) {
    log(Event::GlobalInstallFound { path: &path });
    Some(Executable(path))
  } else {
    log(Event::GlobalInstallNotFound);
    None
  }
}
