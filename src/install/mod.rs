//! the different ways to install an application

pub mod compile_go;
pub mod compile_rust;
pub mod download_archive;
pub mod download_executable;
pub mod other_app_folder;

pub use compile_go::CompileGo;
pub use compile_rust::CompileRust;
pub use download_archive::DownloadArchive;
pub use download_executable::DownloadExecutable;
pub use other_app_folder::OtherAppFolder;

use crate::config::{AppName, Version};
use crate::output::Output;
use crate::platform::Platform;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::Result;

/// the different methods to install an application
pub enum Method<'a> {
    /// installs the application by downloading and extracting an archive containing the application executable from the internet
    DownloadArchive(&'a dyn download_archive::DownloadArchive),
    /// installs the application by downloading the pre-compiled executable from the internet
    DownloadExecutable(&'a dyn DownloadExecutable),
    /// installs the applications by compiling it from its source written in Go
    CompileGoSource(&'a dyn compile_go::CompileGo),
    /// installs the application by compiling it from its source written in Rust
    CompileRustSource(&'a dyn compile_rust::CompileRust),
    /// this application is shipped as part of the given other application
    InstallAnotherApp(&'a dyn OtherAppFolder),
}

impl<'a> Method<'a> {
    pub fn executable_location(&self, version: &Version, platform: Platform) -> String {
        match self {
            Method::DownloadArchive(app) => app.executable_path_in_archive(version, platform),
            Method::DownloadExecutable(app) => app.executable_filename(platform),
            Method::CompileGoSource(app) => app.executable_filename(platform),
            Method::CompileRustSource(app) => app.executable_filename(platform),
            Method::InstallAnotherApp(app) => app.executable_path_in_other_app_yard(version, platform),
        }
    }

    pub fn yard_app(&self) -> AppName {
        match self {
            Method::DownloadArchive(app) => app.name(),
            Method::DownloadExecutable(app) => app.name(),
            Method::CompileGoSource(app) => app.name(),
            Method::CompileRustSource(app) => app.name(),
            Method::InstallAnotherApp(app) => app.app_to_install().name(),
        }
    }
}

pub fn install(install_methods: Vec<Method>, version: &Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    for install_method in install_methods {
        let result = match install_method {
            Method::DownloadArchive(app) => download_archive::run(app, version, platform, output),
            Method::DownloadExecutable(app) => download_executable::install(app, version, platform, output),
            Method::CompileGoSource(app) => compile_go::run(app, version, output),
            Method::CompileRustSource(app) => compile_rust::run(app, version),
            Method::InstallAnotherApp(app) => other_app_folder::install_other_app(app, version, platform, output),
        }?;
        if result {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn load(install_methods: Vec<Method>, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
    for installation_method in install_methods {
        let yard_app_name = installation_method.yard_app();
        let location_in_yard = installation_method.executable_location(version, platform);
        let fullpath = yard.app_folder(&yard_app_name, version).join(location_in_yard);
        if fullpath.exists() {
            return Some(Executable(fullpath));
        }
    }
    None
}
