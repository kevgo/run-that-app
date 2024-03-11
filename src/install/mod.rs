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
pub use compile_go::CompileFromGoSource;
pub use compile_rust::CompileFromRustSource;
pub use download_archive::DownloadArchive;
pub use download_executable::DownloadExecutable;
pub use other_app_folder::OtherAppFolder;

/// the different methods to install an application
pub enum Method<'a> {
    DownloadArchive(&'a dyn DownloadArchive),
    DownloadExecutable(&'a dyn DownloadExecutable),
    CompileGoSource(&'a dyn CompileFromGoSource),
    CompileRustSource(&'a dyn CompileFromRustSource),
    InstallAnotherApp(&'a dyn OtherAppFolder),
}

pub fn install(install_methods: Vec<Method>, version: &Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    for install_method in install_methods {
        let result = match install_method {
            Method::DownloadArchive(app) => download_archive::install(app, version, platform, output),
            Method::DownloadExecutable(app) => download_executable::install(app, version, platform, output),
            Method::CompileGoSource(app) => compile_go::compile_go(app, version, output),
            Method::CompileRustSource(app) => compile_rust::compile_rust(app, version),
            Method::InstallAnotherApp(app) => other_app_folder::install_other_app(app, version, platform, output),
        }?;
        if result {
            return Ok(true);
        }
    }
    Ok(false)
}
