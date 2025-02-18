use crate::logging::{Event, Log};
use crate::executable::{ExecutableFile, ExecutableNamePlatform};
use which::which_global;

pub(crate) fn find_global_install(binary_name: &ExecutableNamePlatform, log: Log) -> Option<ExecutableFile> {
  log(Event::GlobalInstallSearch { binary: binary_name });
  if let Ok(path) = which_global(binary_name.as_ref()) {
    log(Event::GlobalInstallFound { path: &path });
    Some(ExecutableFile::from(path))
  } else {
    log(Event::GlobalInstallNotFound);
    None
  }
}
