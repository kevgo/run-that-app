//! the different ways to install an application

pub mod compile_go;
pub mod compile_rust;
mod download_executable;

pub use download_executable::{download_executable, ArtifactType, DownloadArgs};
