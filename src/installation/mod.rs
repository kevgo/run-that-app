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
use std::fmt::Debug;
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
      Method::ExecutableInAnotherApp { other_app, executable_path } => {
        executable_in_another_app::executable_path(other_app.as_ref(), version, yard, executable_path)
      }
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

// need to implement this manually because the ExecutableInAnotherApp::other_app is not debuggable per se
impl Debug for Method {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::DownloadArchive { url, path_in_archive } => f
        .debug_struct("DownloadArchive")
        .field("url", url)
        .field("path_in_archive", path_in_archive)
        .finish(),
      Self::DownloadExecutable { url } => f.debug_struct("DownloadExecutable").field("url", url).finish(),
      Self::CompileGoSource { import_path } => f.debug_struct("CompileGoSource").field("import_path", import_path).finish(),
      Self::CompileRustSource { crate_name, filepath } => f
        .debug_struct("CompileRustSource")
        .field("crate_name", crate_name)
        .field("filepath", filepath)
        .finish(),
      Self::ExecutableInAnotherApp { other_app, executable_path } => f
        .debug_struct("ExecutableInAnotherApp")
        .field("other_app", &other_app.name())
        .field("executable_path", executable_path)
        .finish(),
    }
  }
}

// need to implement this manually because the ExecutableInAnotherApp::other_app is not comparable
impl PartialEq for Method {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (
        Self::DownloadArchive {
          url: l_url,
          path_in_archive: l_path_in_archive,
        },
        Self::DownloadArchive {
          url: r_url,
          path_in_archive: r_path_in_archive,
        },
      ) => l_url == r_url && l_path_in_archive == r_path_in_archive,
      (Self::DownloadExecutable { url: l_url }, Self::DownloadExecutable { url: r_url }) => l_url == r_url,
      (Self::CompileGoSource { import_path: l_import_path }, Self::CompileGoSource { import_path: r_import_path }) => l_import_path == r_import_path,
      (
        Self::CompileRustSource {
          crate_name: l_crate_name,
          filepath: l_filepath,
        },
        Self::CompileRustSource {
          crate_name: r_crate_name,
          filepath: r_filepath,
        },
      ) => l_crate_name == r_crate_name && l_filepath == r_filepath,
      (
        Self::ExecutableInAnotherApp {
          other_app: l_other_app,
          executable_path: l_executable_path,
        },
        Self::ExecutableInAnotherApp {
          other_app: r_other_app,
          executable_path: r_executable_path,
        },
      ) => l_other_app.name() == r_other_app.name() && l_executable_path == r_executable_path,
      _ => false,
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
    } => download_archive::run(app, version, archive_url, path_in_archive, optional, yard, log),
    Method::DownloadExecutable { url: download_url } => download_executable::run(app, download_url, version, platform, optional, yard, log),
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
