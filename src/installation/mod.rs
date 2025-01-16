//! the different ways to install an application

mod compile_go;
mod compile_rust;
mod download_archive;
mod download_executable;
mod executable_in_another_app;

use crate::applications::App;
use crate::configuration::{self, Version};
use crate::logging::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::yard::Yard;
use std::path::PathBuf;

/// the different methods to install an application
pub enum Method {
  /// installs the application by downloading and extracting an archive containing the application executable from the internet
  DownloadArchive {
    /// the URL of the archive to download
    url: String,
    /// relative path of the executable inside the archive
    path_in_archive: String,
  },

  /// installs the application by downloading the pre-compiled executable from the internet
  DownloadExecutable {
    /// the URL of the executable to download
    url: String,
  },

  /// installs the applications by compiling it from its source written in Go
  CompileGoSource {
    /// the Go import path to use
    import_path: String,
  },

  /// installs the application by compiling it from its source written in Rust
  CompileRustSource {
    /// the name of the Rust crate that contains the executable
    crate_name: &'static str,
    /// the executable path within the yard
    filepath: String,
  },

  /// this application is shipped as part of another application
  ExecutableInAnotherApp { other_app: Box<dyn App>, executable_path: String },
}

impl Method {
  /// provides the location of this app's executable within its yard
  pub fn executable_location(&self, app: &dyn App, version: &Version, platform: Platform, yard: &Yard) -> PathBuf {
    match self {
      Method::DownloadArchive { url: _, path_in_archive } => yard.app_folder(&app.name(), version).join(path_in_archive),
      Method::DownloadExecutable { url: _ } => yard.app_folder(&app.name(), version).join(app.executable_filename(platform)),
      Method::CompileGoSource { import_path: _ } => compile_go::executable_path(app, version, platform, yard),
      Method::CompileRustSource { crate_name: _, filepath } => compile_rust::executable_path(app, version, yard, filepath),
      Method::ExecutableInAnotherApp {
        other_app: app_to_install,
        executable_path,
      } => executable_in_another_app::executable_path(app_to_install.as_ref(), version, yard, executable_path),
    }
  }

  pub fn name(&self, app: &str, version: &Version) -> String {
    match self {
      Method::DownloadArchive { url: _, path_in_archive: _ } => format!("download archive for {app}@{version}"),
      Method::DownloadExecutable { url: _ } => format!("download executable for {app}@{version}"),
      Method::CompileGoSource { import_path: _ } | Method::CompileRustSource { crate_name: _, filepath: _ } => format!("compile {app}@{version} from source"),
      Method::ExecutableInAnotherApp {
        other_app: app_to_install,
        executable_path: _,
      } => format!("install {app}@{version} through {carrier}", carrier = app_to_install.name()),
    }
  }
}

/// installs the given app using the first of the given installation methods that works
pub fn any(app: &dyn App, version: &Version, platform: Platform, optional: bool, yard: &Yard, config_file: &configuration::File, log: Log) -> Result<Outcome> {
  for install_method in app.install_methods(version, platform) {
    let outcome = install(app, &install_method, version, platform, optional, yard, config_file, log)?;
    if outcome.success() {
      return Ok(outcome);
    }
  }
  Ok(Outcome::NotInstalled)
}

/// installs the given app using the given installation method
pub fn install(
  app: &dyn App,
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
      url: archive_url,
      path_in_archive,
    } => download_archive::run(app, version, archive_url, &path_in_archive, optional, yard, log),
    Method::DownloadExecutable { url: download_url } => download_executable::install(app, download_url, version, platform, optional, yard, log),
    Method::CompileGoSource { import_path } => compile_go::run(app, import_path, platform, version, optional, config_file, yard, log),
    Method::CompileRustSource { crate_name, filepath } => compile_rust::run(app, crate_name, version, yard, filepath, log),
    Method::ExecutableInAnotherApp {
      other_app: app_to_install,
      executable_path,
    } => executable_in_another_app::install_other_app(app_to_install.as_ref(), version, platform, optional, yard, executable_path, config_file, log),
  }
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
  Installed { executable: Executable },
  NotInstalled,
}

impl Outcome {
  pub fn success(&self) -> bool {
    match self {
      Outcome::Installed { executable: _ } => true,
      Outcome::NotInstalled => false,
    }
  }
}
