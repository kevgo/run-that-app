//! the different ways to install an application

pub mod compile_go;
pub mod compile_rust;
pub mod download_archive;
pub mod download_executable;
pub mod other_app_folder;

use crate::config::Version;
use crate::output::Output;
use crate::platform::Platform;
use crate::Result;
pub use compile_go::CompileGo;
pub use download_archive::DownloadArchive;
pub use download_executable::DownloadExecutable;
pub use other_app_folder::OtherAppFolder;

/// the different methods to install an application
pub enum Method<'a> {
    /// installs the application by downloading and extracting an archive containing the application executable from the internet
    DownloadArchive(&'a dyn download_archive::DownloadArchive),
    /// installs the application by downloading the pre-compiled executable from the internet
    DownloadExecutable(&'a dyn DownloadExecutable),
    /// installs the applications by compiling it from its source written in Go
    CompileGoSource(&'a dyn compile_go::CompileGo),
    /// installs the application by compiling it from its source written in Rust
    CompileRustSource(&'a dyn compile_rust::Data),
    /// this application is shipped as part of the given other application
    InstallAnotherApp(&'a dyn OtherAppFolder),
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
