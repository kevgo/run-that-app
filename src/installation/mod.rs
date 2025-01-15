//! the different ways to install an application

mod compile_go;
mod compile_rust;
mod download_archive;
mod download_executable;
mod executable_in_another_app;

use std::path::PathBuf;

use crate::configuration::{self, ApplicationName, Version};
use crate::logging::{Event, Log};
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::{CallSignature, Executable};
use crate::yard::Yard;
pub use compile_go::CompileGoSource;
pub use compile_rust::CompileRustSource;
pub use download_archive::DownloadArchive;
pub use download_executable::DownloadExecutable;
pub use executable_in_another_app::ExecutableInAnotherApp;

/// the different methods to install an application
pub enum Method<'a> {
  /// installs the application by downloading and extracting an archive containing the application executable from the internet
  DownloadArchive(&'a dyn DownloadArchive),
  /// installs the application by downloading the pre-compiled executable from the internet
  DownloadExecutable(&'a dyn DownloadExecutable),
  /// installs the applications by compiling it from its source written in Go
  CompileGoSource(&'a dyn CompileGoSource),
  /// installs the application by compiling it from its source written in Rust
  CompileRustSource(&'a dyn CompileRustSource),
  /// this application is shipped as part of another application
  ExecutableInAnotherApp(&'a dyn ExecutableInAnotherApp),
}

impl Method<'_> {
  /// provides the location of this app's executable within its yard
  pub fn executable_location(&self, version: &Version, platform: Platform) -> CallSignature<PathBuf> {
    match self {
      Method::DownloadArchive(app) => CallSignature { executable: (), arguments: () } app.executable_path_in_archive(version, platform),
      Method::DownloadExecutable(app) => app.executable_filename(platform),
      Method::CompileGoSource(app) => app.executable_filename(platform),
      Method::CompileRustSource(app) => app.executable_path_in_folder(platform),
      Method::ExecutableInAnotherApp(app) => app.executable_path_in_other_app_yard(version, platform),
    }
  }

  /// provides the name of the application in whose yard this app is installed
  pub fn yard_app(&self) -> ApplicationName {
    match self {
      Method::DownloadArchive(app) => app.name(),
      Method::DownloadExecutable(app) => app.name(),
      Method::CompileGoSource(app) => app.name(),
      Method::CompileRustSource(app) => app.name(),
      Method::ExecutableInAnotherApp(app) => app.app_to_install().name(),
    }
  }

  pub fn name(&self, version: &Version) -> String {
    match self {
      Method::DownloadArchive(app) => format!("download archive for {app}@{version}", app = app.name()),
      Method::DownloadExecutable(app) => format!("download executable for {app}@{version}", app = app.name()),
      Method::CompileGoSource(app) => format!("compile {app}@{version} from source", app = app.name()),
      Method::CompileRustSource(app) => format!("compile {app}@{version} from source", app = app.name()),
      Method::ExecutableInAnotherApp(app) => format!(
        "install {app}@{version} through {carrier}",
        app = app.name(),
        carrier = app.app_to_install().name()
      ),
    }
  }
}

/// installs an app using the first of its installation methods that works
pub fn any(
  install_methods: Vec<Method>,
  version: &Version,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  config_file: &configuration::File,
  log: Log,
) -> Result<Outcome> {
  for install_method in install_methods {
    if install(&install_method, version, platform, optional, yard, config_file, log)?.success() {
      return Ok(Outcome::Installed);
    }
  }
  Ok(Outcome::NotInstalled)
}

pub fn install(
  install_method: &Method,
  version: &Version,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  config_file: &configuration::File,
  log: Log,
) -> Result<Outcome> {
  match install_method {
    Method::DownloadArchive(app) => download_archive::run(*app, version, platform, optional, yard, log),
    Method::DownloadExecutable(app) => download_executable::install(*app, version, platform, optional, yard, log),
    Method::CompileGoSource(app) => compile_go::run(*app, platform, version, optional, config_file, yard, log),
    Method::CompileRustSource(app) => compile_rust::run(*app, version, yard, log),
    Method::ExecutableInAnotherApp(app) => executable_in_another_app::install_other_app(*app, version, platform, optional, yard, config_file, log),
  }
}

/// assuming one of the given installation methods of an app worked, loads that app's executable
pub fn load(install_methods: Vec<Method>, version: &Version, platform: Platform, yard: &Yard, log: Log) -> Option<Executable> {
  for installation_method in install_methods {
    let yard_app_name = installation_method.yard_app();
    let location_in_yard = installation_method.executable_location(version, platform);
    let fullpath = yard.app_folder(&yard_app_name, version).join(location_in_yard);
    log(Event::YardCheckExistingAppBegin { path: &fullpath });
    if fullpath.exists() {
      log(Event::YardCheckExistingAppFound);
      return Some(Executable(fullpath));
    }
  }
  log(Event::YardCheckExistingAppNotFound);
  None
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
  Installed,
  NotInstalled,
}

impl Outcome {
  pub fn success(&self) -> bool {
    match self {
      Outcome::Installed => true,
      Outcome::NotInstalled => false,
    }
  }
}
