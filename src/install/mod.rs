//! the different ways to install an application

pub mod archive;
pub mod compile_go;
pub mod compile_rust;
pub mod executable;
pub mod other_app_folder;

use crate::config::{RequestedVersion, Version};
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

pub fn install(install_methods: Vec<Method>, version: Version, platform: Platform, output: &dyn Output) -> Result<bool> {
    for install_method in install_methods {
        let result = match install_method {
            Method::DownloadArchive(app) => archive::install(app, &version, platform, output),
            Method::DownloadExecutable(app) => todo!(),
            Method::CompileGoSource(app) => compile_go::compile_go(app, &version, output),
            Method::CompileRustSource(app) => compile_rust::compile_rust(app, &version, platform, output),
            Method::InstallAnotherApp(app) => {
                // get the requested version from the config file
                let app_to_install = app.app_to_install();
                // Note: we know it must be the Yard variant here. At this point we are installing the app.
                // Only Yard variants get installed. The Path variant doesn't get installed.
                let requested_version = RequestedVersion::Yard(version);
                other_app_folder::install_other_app(app, requested_version, platform, output)
            }
        }?;
        if result {
            return Ok(true);
        }
    }
    Ok(false)
}
