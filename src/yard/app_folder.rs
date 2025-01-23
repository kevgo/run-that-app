use crate::applications::AppDefinition;
use crate::configuration::Version;
use crate::logging::{Event, Log};
use crate::platform::Platform;
use crate::run::{ExecutableNameUnix, ExecutablePath};
use std::path::PathBuf;

/// a folder that contains an installed app
#[derive(Debug, PartialEq)]
pub struct AppFolder {
  pub root: PathBuf,
  pub app_definition: Box<dyn AppDefinition>,
}

impl AppFolder {
  /// tries to load the given executable of the given app from the yard
  pub fn load_executable(&self, executable: &ExecutableNameUnix, version: &Version, platform: Platform, log: Log) -> Option<ExecutablePath> {
    for installation_method in self.app_definition.run_method(version, platform).install_methods() {
      let fullpaths = installation_method.executable_paths(&executable.clone().platform_path(platform.os), &self.root);
      for fullpath in fullpaths {
        log(Event::YardCheckExistingAppBegin { path: &fullpath });
        if fullpath.exists() {
          log(Event::YardCheckExistingAppFound);
          return Some(ExecutablePath::from(fullpath));
        }
        log(Event::YardCheckExistingAppNotFound);
      }
    }
    None
  }
}

#[cfg(test)]
mod tests {}
