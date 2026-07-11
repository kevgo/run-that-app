//! This module implements the different ways to download and install an application.

mod compile_go;
mod compile_rust;
mod download_archive;
mod download_executable;
mod install_nodejs_package;

use crate::applications::{AppDefinition, ApplicationName, Apps};
use crate::configuration::{RequestedVersion, RequestedVersions, Version};
use crate::context::RuntimeContext;
use crate::download::Url;
use crate::error::Result;
use crate::executables::ExecutableNamePlatform;
use crate::installation::compile_rust::RustSource;
use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};

/// the different methods to install an application
#[derive(Debug, PartialEq)]
pub enum Method {
  /// installs the application by downloading and extracting an archive containing the application executable from the internet
  DownloadArchive {
    /// the URL of the archive to download
    url: Url,
    /// The possible folders within the archive that might contain the executable files.
    /// Multiple options exist because for some apps, the Windows archive contains a different folder structure than the Linux or macOS archive.
    /// Provide all possible folders here. If the executables are in the root folder of the archive, leave this empty.
    bin_folder: BinFolder,
  },

  /// installs the application by downloading the pre-compiled executable from the internet
  DownloadExecutable {
    /// the URL of the executable to download
    url: Url,
  },

  /// installs an application written in Go by compiling it from its source hosted on a remote repository
  CompileGoSource {
    /// the Go import path to use
    import_path: String,
  },

  /// installs an application written in Rust by compiling it from its source hosted on crates.io
  CompileRustCrate {
    /// the name of the Rust crate that contains the executable
    name: &'static str,
    /// The subfolder that contains the executables after compilation.
    bin_folder: BinFolder,
  },

  /// installs an application written in Rust by compiling it from its source hosted on a remote repository
  CompileRustRepo {
    /// the URL of the repository containing the source code
    url: Url,
  },

  InstallNodeJSPackage {
    /// the name of the `NodeJS` package to install
    package: &'static str,
  },
}

impl Method {
  pub fn bin_folder(self) -> BinFolder {
    match self {
      Method::DownloadExecutable { url: _ } | Method::CompileGoSource { import_path: _ } | Method::InstallNodeJSPackage { package: _ } => BinFolder::Root,
      Method::DownloadArchive { url: _, bin_folder } | Method::CompileRustCrate { name: _, bin_folder } => bin_folder,
      Method::CompileRustRepo { url: _ } => BinFolder::Subfolder { path: "bin".into() },
    }
  }

  /// provides possible locations of the given executable within the given app folder in the given yard
  pub fn executable_paths(&self, app_folder: &Path, executable_filename: &ExecutableNamePlatform) -> Vec<PathBuf> {
    match self {
      Method::DownloadArchive { url: _, bin_folder } => bin_folder.executable_paths(app_folder, executable_filename),
      Method::DownloadExecutable { url: _ } | Method::CompileGoSource { import_path: _ } => vec![app_folder.join(executable_filename)],
      Method::CompileRustCrate { name: _, bin_folder } => match bin_folder {
        BinFolder::Root => vec![app_folder.join(executable_filename)],
        BinFolder::Subfolder { path } => vec![app_folder.join(path).join(executable_filename)],
        BinFolder::Subfolders { options } | BinFolder::RootOrSubfolders { options } => {
          options.iter().map(|option| app_folder.join(option).join(executable_filename)).collect()
        }
      },
      Method::CompileRustRepo { url: _ } => vec![app_folder.join("bin").join(executable_filename)],
      Method::InstallNodeJSPackage { package: _ } => vec![app_folder.join("node_modules").join(".bin").join(executable_filename)],
    }
  }

  pub fn is_from_source(&self) -> bool {
    match self {
      Method::DownloadArchive { url: _, bin_folder: _ } | Method::DownloadExecutable { url: _ } | Method::InstallNodeJSPackage { package: _ } => false,
      Method::CompileGoSource { import_path: _ } | Method::CompileRustCrate { name: _, bin_folder: _ } | Method::CompileRustRepo { url: _ } => true,
    }
  }

  pub fn name(&self, app: &ApplicationName, version: &Version) -> String {
    match self {
      Method::DownloadArchive { url: _, bin_folder: _ } => format!("download archive for {app}@{version}"),
      Method::DownloadExecutable { url: _ } => format!("download executable for {app}@{version}"),
      Method::CompileGoSource { import_path: _ } | Method::CompileRustCrate { name: _, bin_folder: _ } | Method::CompileRustRepo { url: _ } => {
        format!("compile {app}@{version} from source")
      }
      Method::InstallNodeJSPackage { package } => format!("install NodeJS package {package}@{version}"),
    }
  }
}

/// describes the various locations where the executable files could be inside an application folder
#[derive(Clone, Debug, PartialEq)]
pub enum BinFolder {
  /// all executables are directly in the app folder
  Root,
  /// the executables are in the given subfolder
  Subfolder { path: PathBuf },
  /// the executables are in one of the given subfolders
  Subfolders { options: Vec<PathBuf> },
  /// the executables are either directly in the app folder or in one of the given subfolders
  RootOrSubfolders { options: Vec<PathBuf> },
}

impl BinFolder {
  pub fn possible_paths(&self, app_folder: &Path) -> Vec<PathBuf> {
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

  pub fn executable_paths(&self, app_folder: &Path, executable_name: &ExecutableNamePlatform) -> Vec<PathBuf> {
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
      BinFolder::Subfolder { path } => write!(f, "subfolder {}", path.to_string_lossy()),
      BinFolder::Subfolders { options } => {
        let paths: Vec<_> = options.iter().map(|p| p.to_string_lossy()).collect();
        write!(f, "subfolders {}", paths.join(", "))
      }
      BinFolder::RootOrSubfolders { options } => {
        let paths: Vec<_> = options.iter().map(|p| p.to_string_lossy()).collect();
        write!(f, "root or subfolders {}", paths.join(", "))
      }
    }
  }
}

pub fn versions(
  app_definition: &dyn AppDefinition,
  versions: &RequestedVersions,
  optional: bool,
  from_source: bool,
  ctx: &RuntimeContext,
  apps: &Apps,
) -> Result<Outcome> {
  for version in versions {
    match version {
      RequestedVersion::Path(_version_req) => {
        // we can't install anything into the global path
      }
      RequestedVersion::Yard(version) => match version_any_method(app_definition, version, optional, from_source, ctx, apps)? {
        Outcome::Installed => return Ok(Outcome::Installed),
        Outcome::NotInstalled { app: _ } => {}
      },
    }
  }
  Ok(Outcome::NotInstalled {
    app: app_definition.name().clone(),
  })
}

/// installs the given app using any of its installation methods
pub fn version_any_method(
  app_definition: &dyn AppDefinition,
  version: &Version,
  optional: bool,
  from_source: bool,
  ctx: &RuntimeContext,
  apps: &Apps,
) -> Result<Outcome> {
  for install_method in app_definition.run_method(version, ctx.platform).install_methods() {
    if from_source && !install_method.is_from_source() {
      continue;
    }
    match version_method(app_definition, &install_method, version, optional, ctx, apps)? {
      Outcome::Installed => return Ok(Outcome::Installed),
      Outcome::NotInstalled { app: _ } => {}
    }
  }
  let app_name = app_definition.name();
  ctx.yard.mark_not_installable(&app_name, version)?;
  Ok(Outcome::NotInstalled { app: app_name })
}

/// installs the given app using the given installation method
pub fn version_method(
  app_definition: &dyn AppDefinition,
  install_method: &Method,
  version: &Version,
  optional: bool,
  ctx: &RuntimeContext,
  apps: &Apps,
) -> Result<Outcome> {
  let staging_folder = ctx.yard.create_staging_folder(&app_definition.name(), version)?;
  let outcome = match install_method {
    Method::DownloadArchive { url, bin_folder } => download_archive::run(app_definition, &staging_folder, version, url, bin_folder, optional, ctx),
    Method::DownloadExecutable { url: download_url } => download_executable::run(app_definition, &staging_folder, version, download_url, optional, ctx),
    Method::CompileGoSource { import_path } => compile_go::run(&staging_folder, import_path, optional, ctx, apps),
    Method::CompileRustCrate { name, bin_folder: _ } => compile_rust::run(app_definition, version, &staging_folder, &RustSource::CratesIo { name }, ctx.log),
    Method::CompileRustRepo { url } => compile_rust::run(app_definition, version, &staging_folder, &RustSource::Repository { url: url.clone() }, ctx.log),
    Method::InstallNodeJSPackage { package } => install_nodejs_package::run(package, &staging_folder, version, optional, apps),
  }?;
  match outcome {
    Outcome::Installed => {
      let app_folder_path = ctx.yard.app_folder(&app_definition.name(), version);
      ctx.yard.move_staging_folder_to_app_folder(staging_folder, app_folder_path)?;
      Ok(Outcome::Installed)
    }
    Outcome::NotInstalled { app } => Ok(Outcome::NotInstalled { app }),
  }
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
  Installed,
  NotInstalled { app: ApplicationName },
}

impl Outcome {
  pub fn success(&self) -> bool {
    match self {
      Outcome::Installed => true,
      Outcome::NotInstalled { app: _ } => false,
    }
  }
}
