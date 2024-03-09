//! the different ways to install an application

pub mod archive;
pub mod compile_go;
pub mod compile_rust;
pub mod executable;

pub use archive::InstallByArchive;
pub use compile_go::CompileFromGoSource;

/// the different methods to install an application
pub enum Method<'a> {
    DownloadArchive { app: &'a dyn InstallByArchive },
    DownloadExecutable,
    CompileGoSource { app: &'a dyn CompileFromGoSource },
    CompileRustSource,
}
