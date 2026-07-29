use crate::applications::AppDefinition;
use crate::executables::{Executable, ExecutableNamePlatform};
use crate::logging::{Event, Log};
use which::which_global;

pub fn find_global_install(app_to_install: &dyn AppDefinition, binary_name: &ExecutableNamePlatform, log: Log) -> Option<Executable> {
  log(Event::GlobalInstallSearch { binary: binary_name.as_ref() });
  let Ok(path) = which_global(binary_name.as_ref()) else {
    log(Event::GlobalInstallNotFound);
    return None;
  };
  match app_to_install.run_method(version, platform)
  log(Event::GlobalInstallFound { path: &path });
  Some(Executable::from(path))
}
