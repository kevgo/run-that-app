//! the different ways to install an application

pub mod archive;
pub mod compile_go;
pub mod compile_rust;
pub mod executable;
pub mod other_app_folder;

use crate::config::Version;
use crate::output::Output;
use crate::platform::Platform;
use crate::Result;
pub use archive::InstallByArchive;
pub use compile_go::CompileFromGoSource;
pub use compile_rust::CompileFromRustSource;
pub use executable::DownloadExecutable;
pub use other_app_folder::OtherAppFolder;

/// the different methods to install an application
pub enum Method<'a> {
    DownloadArchive(&'a dyn InstallByArchive),
    DownloadExecutable(&'a dyn DownloadExecutable),
    CompileGoSource(&'a dyn CompileFromGoSource),
    CompileRustSource(&'a dyn CompileFromRustSource),
    InstallAnotherApp(&'a dyn OtherAppFolder),
}

pub fn install(install_methods: Vec<Method>, version: &Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    for install_method in install_methods {
        let result = match install_method {
            Method::DownloadArchive(app) => archive::install(app, version, platform, output),
            Method::DownloadExecutable(app) => executable::install(app, version, platform, output),
            Method::CompileGoSource(app) => compile_go::compile_go(app, version, output),
            Method::CompileRustSource(app) => compile_rust::compile_rust(app, version, platform),
            Method::InstallAnotherApp(app) => other_app_folder::install_other_app(app, version, platform, output),
        }?;
        if result {
            return Ok(true);
        }
    }
    Ok(false)
}
