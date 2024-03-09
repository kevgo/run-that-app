//! the different ways to install an application

pub mod archive;
pub mod compile_go;
pub mod compile_rust;
pub mod executable;

use crate::config::Version;
use crate::output::Output;
use crate::platform::Platform;
use crate::Result;
pub use archive::InstallByArchive;
pub use compile_go::CompileFromGoSource;

/// the different methods to install an application
pub enum Method<'a> {
    DownloadArchive(&'a dyn InstallByArchive),
    DownloadExecutable(&'a dyn DownloadExecutable),
    CompileGoSource(&'a dyn CompileFromGoSource),
    CompileRustSource,
}

pub fn install(install_methods: Vec<Method>, version: &Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    for install_method in install_methods {
        let result = match install_method {
            Method::DownloadArchive(app) => archive::install(app, version, platform, output),
            Method::DownloadExecutable(app) => todo!(),
            Method::CompileGoSource(app) => compile_go::compile_go(app, version, output),
            Method::CompileRustSource => todo!(),
        }?;
        if result {
            return Ok(true);
        }
    }
    Ok(false)
}
