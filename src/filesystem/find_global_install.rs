use crate::logging::{Event, Log};
use crate::run::{Executable, ExecutableFilename};
use which::which_global;

pub fn find_global_install(binary_name: &ExecutableFilename, log: Log) -> Option<Executable> {
  log(Event::GlobalInstallSearch { binary: binary_name });
  if let Ok(path) = which_global(binary_name.as_ref()) {
    log(Event::GlobalInstallFound { path: &path });
    Some(Executable::from(path))
  } else {
    log(Event::GlobalInstallNotFound);
    None
  }
}
