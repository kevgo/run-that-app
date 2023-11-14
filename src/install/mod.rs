//! the different ways to install an application

mod compile_from_go_source;
mod compile_from_rust_source;
mod download_precompiled_binary;

use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
pub use compile_from_go_source::CompileFromGoSource;
pub use compile_from_rust_source::CompileFromRustSource;
pub use download_precompiled_binary::DownloadPrecompiledBinary;

pub trait InstallationMethod {
    /// A particular way of installing an application.
    /// Applications typically provide multiple ways of being installed,
    /// i.e. download from GitHub Releases and if that doesn't work, compile from source.
    ///
    /// success --> Ok(Some(executable))
    /// this installation method is not available -->  Ok(None)
    /// this installation method created an error --> Err(UserError)
    fn install(&self, output: &dyn Output) -> Result<Option<Executable>>;
}