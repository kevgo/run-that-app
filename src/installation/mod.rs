//! the different ways to install an application

mod compile_go;
mod compile_rust;
mod download_archive;
mod download_executable;

use crate::applications::App;
use crate::configuration::{self, Version};
use crate::logging::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::run::ExecutableFilename;
use crate::yard::Yard;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

/// the different methods to install an application
#[derive(Debug, PartialEq)]
pub enum Method {
  /// installs the application by downloading and extracting an archive containing the application executable from the internet
  DownloadArchive {
    /// the URL of the archive to download
    url: String,
    /// The possible folders within the archive that might contain the executable files.
    /// Multiple options exist because for some apps, the Windows archive contains a different folder structure than the Linux or macOS archive.
    /// Provide all possible folders here. If the executables are in the root folder of the archive, leave this empty.
    bin_folder: BinFolder,
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
    /// The subfolder that contains the executables after compilation.
    bin_folder: Option<&'static str>,
  },
}

impl Method {
  /// provides possible locations of the given executable within the given app folder in the given  yard
  pub fn executable_paths(&self, app: &dyn App, executable_filename: &ExecutableFilename, version: &Version, yard: &Yard) -> Vec<PathBuf> {
    let app_folder = yard.app_folder(&app.name(), version);
    match self {
      Method::DownloadArchive {
        url: _,
        bin_folder: bin_folders,
      } => bin_folders.executable_paths(&app_folder, executable_filename),
      Method::DownloadExecutable { url: _ } | Method::CompileGoSource { import_path: _ } => vec![app_folder.join(executable_filename)],
      Method::CompileRustSource { crate_name: _, bin_folder } => vec![match bin_folder {
        Some(bin_folder) => app_folder.join(bin_folder).join(executable_filename),
        None => app_folder.join(executable_filename),
      }],
    }
  }

  pub fn name(&self, app: &str, version: &Version) -> String {
    match self {
      Method::DownloadArchive { url: _, bin_folder: _ } => format!("download archive for {app}@{version}"),
      Method::DownloadExecutable { url: _ } => format!("download executable for {app}@{version}"),
      Method::CompileGoSource { import_path: _ } | Method::CompileRustSource { crate_name: _, bin_folder: _ } => format!("compile {app}@{version} from source"),
    }
  }
}

/// describes the various locations where the executable files could be inside an application folder
#[derive(Debug, PartialEq)]
pub enum BinFolder {
  /// all executables are directly in the app folder
  Root,
  /// the executables are in the given subfolder
  Subfolder { path: String },
  /// the executables are in one of the given subfolders
  Subfolders { options: Vec<String> },
  /// the executables are either directly in the app folder or in one of the given subfolders
  RootOrSubfolders { options: Vec<String> },
}

impl BinFolder {
  pub fn executable_paths(&self, app_folder: &Path, executable_name: &ExecutableFilename) -> Vec<PathBuf> {
    match self {
      BinFolder::RootOrSubfolders { options } => {
        let mut result = vec![app_folder.join(executable_name)];
        for option in options {
          result.push(app_folder.join(option).join(executable_name));
        }
        result
      }
      BinFolder::Root => vec![app_folder.join(executable_name)],
      BinFolder::Subfolder { path } => vec![app_folder.join(path).join(executable_name)],
      BinFolder::Subfolders { options } => {
        let mut result = vec![];
        for option in options {
          result.push(app_folder.join(option).join(executable_name));
        }
        result
      }
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
    Method::DownloadArchive { url, bin_folder: bin_folders } => download_archive::run(app, version, url, bin_folders, optional, platform, yard, log),
    Method::DownloadExecutable { url: download_url } => download_executable::run(app, download_url, version, platform, optional, yard, log),
    Method::CompileGoSource { import_path } => compile_go::run(app, import_path, platform, version, optional, config_file, yard, log),
    Method::CompileRustSource { crate_name, bin_folder: _ } => compile_rust::run(app, crate_name, version, yard, log),
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
