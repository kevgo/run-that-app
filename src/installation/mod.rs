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
use crate::yard::Yard;
use std::fmt::Debug;
use std::path::PathBuf;

/// the different methods to install an application
#[derive(Debug, PartialEq)]
pub enum Method {
  /// installs the application by downloading and extracting an archive containing the application executable from the internet
  DownloadArchive {
    /// the URL of the archive to download
    url: String,
    /// The folders within the archive that might contain executable files.
    /// Provide all possible folders here, they will be tried until the executable is found.
    /// Leave this empty if the executables are in the root folder of the archive.
    bin_folders: Vec<String>,
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
}

impl Method {
  /// provides possible locations of the given executable within the given app folder in the given  yard
  pub fn executable_location(&self, app: &dyn App, executable_name: &str, version: &Version, platform: Platform, yard: &Yard) -> Vec<PathBuf> {
    let app_folder = yard.app_folder(&app.name(), version);
    let executable_filename = format!("{executable_name}{ext}", ext = platform.os.executable_extension());
    let in_root_folder = vec![app_folder.join(&executable_filename)];
    match self {
      Method::DownloadArchive { url: _, bin_folders } => bin_folders
        .into_iter()
        .map(|bin_folder| app_folder.join(bin_folder).join(&executable_filename))
        .collect(),
      Method::DownloadExecutable { url: _ } => in_root_folder,
      Method::CompileGoSource { import_path: _ } => in_root_folder,
      Method::CompileRustSource { crate_name: _, filepath } => vec![app_folder.join(filepath).join(executable_filename)],
    }
  }

  pub fn name(&self, app: &str, version: &Version) -> String {
    match self {
      Method::DownloadArchive { url: _, bin_folders: _ } => format!("download archive for {app}@{version}"),
      Method::DownloadExecutable { url: _ } => format!("download executable for {app}@{version}"),
      Method::CompileGoSource { import_path: _ } | Method::CompileRustSource { crate_name: _, filepath: _ } => format!("compile {app}@{version} from source"),
    }
  }
}

/// installs the given app using the first of the given installation methods that works
pub fn any(app: &dyn App, version: &Version, platform: Platform, optional: bool, yard: &Yard, config_file: &configuration::File, log: Log) -> Result<Outcome> {
  for install_method in app.run_method(version, platform).install_methods() {
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
    Method::DownloadArchive { url, bin_folders } => download_archive::run(app, version, url, bin_folders, optional, yard, log),
    Method::DownloadExecutable { url: download_url } => download_executable::run(app, download_url, version, platform, optional, yard, log),
    Method::CompileGoSource { import_path } => compile_go::run(app, import_path, platform, version, optional, config_file, yard, log),
    Method::CompileRustSource { crate_name, filepath } => compile_rust::run(app, crate_name, version, yard, filepath, log),
  }
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
