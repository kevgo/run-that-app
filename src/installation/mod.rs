//! the different ways to install an application

mod compile_go;
mod compile_rust;
mod download_archive;
mod download_executable;
mod executable_in_another_app;

use crate::applications::App;
use crate::configuration::{self, ApplicationName, Version};
use crate::logging::{Event, Log};
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::yard::Yard;
pub use compile_go::CompileGoSource;
pub use compile_rust::CompileRustSource;
pub use download_archive::DownloadArchive;
pub use download_executable::DownloadExecutable;
pub use executable_in_another_app::ExecutableInAnotherApp;

/// the different methods to install an application
pub enum Method {
  /// installs the application by downloading and extracting an archive containing the application executable from the internet
  DownloadArchive { archive_url: String, executable_path_in_archive: String },

  /// installs the application by downloading the pre-compiled executable from the internet
  DownloadExecutable { download_url: String },

  /// installs the applications by compiling it from its source written in Go
  CompileGoSource { import_path: String },

  /// installs the application by compiling it from its source written in Rust
  CompileRustSource {
    crate_name: &'static str,
    executable_path_in_folder: String,
  },

  /// this application is shipped as part of another application
  ExecutableInAnotherApp {
    app_to_install: Box<dyn App>,
    executable_path_in_other_yard: String,
  },
}

impl Method {
  /// provides the location of this app's executable within its yard
  pub fn executable_location(&self) -> Option<&str> {
    match self {
      Method::DownloadArchive {
        archive_url: _,
        executable_path_in_archive,
      } => Some(executable_path_in_archive),
      Method::DownloadExecutable { download_url: _ } => None,
      Method::CompileGoSource { import_path: _ } => None,
      Method::CompileRustSource {
        crate_name: _,
        executable_path_in_folder,
      } => Some(executable_path_in_folder),
      Method::ExecutableInAnotherApp {
        app_to_install: _,
        executable_path_in_other_yard: _,
      } => None,
    }
  }

  /// provides the name of the application in whose yard this app is installed
  pub fn yard_app(&self) -> Option<&Box<dyn App>> {
    match self {
      Method::DownloadArchive {
        archive_url: _,
        executable_path_in_archive: _,
      } => None,
      Method::DownloadExecutable { download_url: _ } => None,
      Method::CompileGoSource { import_path: _ } => None,
      Method::CompileRustSource {
        crate_name: _,
        executable_path_in_folder: _,
      } => None,
      Method::ExecutableInAnotherApp {
        app_to_install,
        executable_path_in_other_yard: _,
      } => Some(app_to_install),
    }
  }

  pub fn name(&self) -> String {
    match self {
      Method::DownloadArchive {
        archive_url: _,
        executable_path_in_archive: _,
      } => format!("download archive"),
      Method::DownloadExecutable { download_url: _ } => format!("download executable"),
      Method::CompileGoSource { import_path: _ } => format!("compile from Go source"),
      Method::CompileRustSource {
        crate_name: _,
        executable_path_in_folder: _,
      } => format!("compile from Rust source"),
      Method::ExecutableInAnotherApp {
        app_to_install,
        executable_path_in_other_yard: _,
      } => format!("install through {}", app_to_install.name()),
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
    Method::DownloadArchive {
      archive_url,
      executable_path_in_archive,
    } => download_archive::run(*app, version, platform, optional, yard, log),
    Method::DownloadExecutable { download_url } => download_executable::install(*app, version, platform, optional, yard, log),
    Method::CompileGoSource { import_path } => compile_go::run(*app, platform, version, optional, config_file, yard, log),
    Method::CompileRustSource {
      crate_name,
      executable_path_in_folder,
    } => compile_rust::run {},
    Method::ExecutableInAnotherApp {
      app_to_install,
      executable_path_in_other_yard,
    } => executable_in_another_app::install_other_app(*app, version, platform, optional, yard, config_file, log),
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
