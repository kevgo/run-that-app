//! the different ways to install an application

mod compile_go;
mod compile_rust;
mod download_archive;
mod download_executable;
mod executable_in_another_app;

use std::fmt::Display;
use std::path::PathBuf;

use crate::applications::App;
use crate::commands::run::load_or_install;
use crate::configuration::{self, RequestedVersion, Version};
use crate::logging::{Event, Log};
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::yard::Yard;

/// the different methods to install an application
pub enum Method {
  /// installs the application by downloading and extracting an archive containing the application executable from the internet
  // TODO: rename to url                  rename to executable_path
  DownloadArchive { archive_url: String, executable_path_in_archive: String },

  /// installs the application by downloading the pre-compiled executable from the internet
  // TODO:              rename to url
  DownloadExecutable { download_url: String },

  /// installs the applications by compiling it from its source written in Go
  CompileGoSource { import_path: String },

  /// installs the application by compiling it from its source written in Rust
  CompileRustSource {
    crate_name: &'static str,
    /// the executable path within the yard
    // TODO: rename to executable_path
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
  pub fn executable_location(&self, app: &dyn App, version: &Version, platform: Platform, yard: &Yard) -> PathBuf {
    match self {
      Method::DownloadArchive {
        archive_url: _,
        executable_path_in_archive,
      } => yard.app_folder(&app.name(), version).join(executable_path_in_archive),
      Method::DownloadExecutable { download_url: _ } => yard.app_folder(&app.name(), version).join(app.executable_filename(platform)),
      Method::CompileGoSource { import_path: _ } => yard.app_folder(&app.name(), version).join(app.executable_filename(platform)),
      Method::CompileRustSource {
        crate_name: _,
        executable_path_in_folder,
      } => yard.app_folder(&app.name(), version).join(executable_path_in_folder),
      Method::ExecutableInAnotherApp {
        app_to_install,
        executable_path_in_other_yard,
      } => yard.app_folder(&app_to_install.name(), version).join(executable_path_in_other_yard),
    }
  }
  fn name(&self, version: &Version) -> String {
    match self {
      Method::DownloadArchive {
        archive_url: _,
        executable_path_in_archive: _,
      } => format!("download archive for {}"),

      Method::DownloadExecutable { download_url: _ } => format!("download executable"),
      Method::CompileGoSource { import_path: _ } => format!("compile from Go source"),
      Method::CompileRustSource {
        crate_name: _,
        executable_path_in_folder: _,
      } => format!("compile from Rust source"),
      Method::ExecutableInAnotherApp {
        app_to_install,
        executable_path_in_other_yard: _,
      } => {
        format!("install through {}", app_to_install.name())
      }
    }
  }
}

/// installs the given app using the first of the given installation methods that works
// TODO: return the installation method used, so that we don't need to detect it later. Or - even better - return the installed executable because that's what we really need later.
pub fn any(app: &dyn App, version: &Version, platform: Platform, optional: bool, yard: &Yard, config_file: &configuration::File, log: Log) -> Result<Outcome> {
  for install_method in app.install_methods(version, platform) {
    if install(app, &install_method, version, platform, optional, yard, config_file, log)?.success() {
      return Ok(Outcome::Installed);
    }
  }
  Ok(Outcome::NotInstalled)
}

/// installs the given app using the given installation method
// TODO: rename to "one" to complement "any"?
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
      archive_url,
      executable_path_in_archive,
    } => download_archive::run(app, version, archive_url, executable_path_in_archive, optional, yard, log),
    Method::DownloadExecutable { download_url } => download_executable::install(app, download_url, version, platform, optional, yard, log),
    Method::CompileGoSource { import_path } => compile_go::run(app, import_path, platform, version, optional, config_file, yard, log),
    Method::CompileRustSource {
      crate_name,
      executable_path_in_folder: _,
    } => compile_rust::run(app, &crate_name, version, yard, log),
    Method::ExecutableInAnotherApp {
      app_to_install,
      executable_path_in_other_yard: _,
    } => {
      load_or_install(
        app_to_install.as_ref(),
        &RequestedVersion::Yard(version.to_owned()),
        platform,
        optional,
        yard,
        config_file,
        log,
      )?;
      Ok(Outcome::Installed)
    }
  }
}

/// tries to load the executable of the given app from the yard
// TODO: move into "yard" module
pub fn load(app: &dyn App, version: &Version, platform: Platform, yard: &Yard, log: Log) -> Option<Executable> {
  for installation_method in app.install_methods(version, platform) {
    let fullpath = installation_method.executable_location(app, version, platform, yard);
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
