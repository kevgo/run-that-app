//! the different ways to install an application

mod compile_from_go_source;
mod compile_from_rust_source;
mod download_precompiled_binary;

pub use compile_from_go_source::{compile_from_go_source, CompileFromGoSource};
pub use compile_from_rust_source::{compile_from_rust_source, CompileFromRustSourceArgs};
pub use download_precompiled_binary::{download_precompiled_binary, ArtifactType, DownloadPrecompiledBinary};
