//! This module implements the different ways to download and install an application.

mod compile_go;
mod compile_rust;
mod download_archive;
mod download_executable;

use crate::applications::AppDefinition;
use crate::configuration::{self, Version};
use crate::logging::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::executable::ExecutableNamePlatform;
use crate::yard::Yard;
use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};

/// the different methods to install an application
#[derive(Debug, PartialEq)]
pub(crate) enum Method {
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
    bin_folder: BinFolder,
  },
}

impl Method {
  pub(crate) fn bin_folder(self) -> BinFolder {
    match self {
      Method::DownloadExecutable { url: _ } | Method::CompileGoSource { import_path: _ } => BinFolder::Root,
      Method::DownloadArchive { url: _, bin_folder } | Method::CompileRustSource { crate_name: _, bin_folder } => bin_folder,
    }
  }

  /// provides possible locations of the given executable within the given app folder in the given  yard
  pub(crate) fn executable_paths(&self, app_folder: &Path, executable_filename: &ExecutableNamePlatform) -> Vec<PathBuf> {
    match self {
      Method::DownloadArchive { url: _, bin_folder } => bin_folder.executable_paths(app_folder, executable_filename),
      Method::DownloadExecutable { url: _ } | Method::CompileGoSource { import_path: _ } => vec![app_folder.join(executable_filename)],
      Method::CompileRustSource { crate_name: _, bin_folder } => match bin_folder {
        BinFolder::Root => vec![app_folder.join(executable_filename)],
        BinFolder::Subfolder { path } => vec![app_folder.join(path).join(executable_filename)],
        BinFolder::Subfolders { options } | BinFolder::RootOrSubfolders { options } => {
          options.iter().map(|option| app_folder.join(option).join(executable_filename)).collect()
        }
      },
    }
  }

  pub(crate) fn name(&self, app: &str, version: &Version) -> String {
    match self {
      Method::DownloadArchive { url: _, bin_folder: _ } => format!("download archive for {app}@{version}"),
      Method::DownloadExecutable { url: _ } => format!("download executable for {app}@{version}"),
      Method::CompileGoSource { import_path: _ } | Method::CompileRustSource { crate_name: _, bin_folder: _ } => format!("compile {app}@{version} from source"),
    }
  }
}

/// describes the various locations where the executable files could be inside an application folder
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum BinFolder {
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
  pub(crate) fn possible_paths(&self, app_folder: &Path) -> Vec<PathBuf> {
    match self {
      BinFolder::Root => vec![app_folder.to_path_buf()],
      BinFolder::Subfolder { path } => vec![app_folder.join(path)],
      BinFolder::Subfolders { options } => options.iter().map(|option| app_folder.join(option)).collect(),
      BinFolder::RootOrSubfolders { options } => {
        let mut result = vec![app_folder.to_path_buf()];
        for option in options {
          result.push(app_folder.join(option));
        }
        result
      }
    }
  }

  pub(crate) fn executable_paths(&self, app_folder: &Path, executable_name: &ExecutableNamePlatform) -> Vec<PathBuf> {
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

impl Display for BinFolder {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("BinFolder: ")?;
    match self {
      BinFolder::Root => write!(f, "root"),
      BinFolder::Subfolder { path } => write!(f, "subfolder {path}"),
      BinFolder::Subfolders { options } => write!(f, "subfolders {}", options.join(", ")),
      BinFolder::RootOrSubfolders { options } => write!(f, "root or subfolders {}", options.join(", ")),
    }
  }
}

/// installs the given app using the first of the given installation methods that works
pub(crate) fn any(
  app_definition: &dyn AppDefinition,
  version: &Version,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  config_file: &configuration::File,
  log: Log,
) -> Result<Outcome> {
  for install_method in app_definition.run_method(version, platform).install_methods() {
    let outcome = install(app_definition, &install_method, version, platform, optional, yard, config_file, log)?;
    if outcome.success() {
      return Ok(outcome);
    }
  }
  Ok(Outcome::NotInstalled)
}

/// installs the given app using the given installation method
pub(crate) fn install(
  app_definition: &dyn AppDefinition,
  install_method: &Method,
  version: &Version,
  platform: Platform,
  optional: bool,
  yard: &Yard,
  config_file: &configuration::File,
  log: Log,
) -> Result<Outcome> {
  let app_folder = yard.create_app_folder(&app_definition.app_name(), version)?;
  match install_method {
    Method::DownloadArchive { url, bin_folder } => download_archive::run(app_definition, &app_folder, version, url, bin_folder, optional, platform, log),
    Method::DownloadExecutable { url: download_url } => download_executable::run(app_definition, &app_folder, download_url, platform, optional, log),
    Method::CompileGoSource { import_path } => compile_go::run(&app_folder, import_path, platform, optional, config_file, yard, log),
    Method::CompileRustSource { crate_name, bin_folder: _ } => compile_rust::run(&app_folder, crate_name, log),
  }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Outcome {
  Installed,
  NotInstalled,
}

impl Outcome {
  pub(crate) fn success(&self) -> bool {
    match self {
      Outcome::Installed => true,
      Outcome::NotInstalled => false,
    }
  }
}
