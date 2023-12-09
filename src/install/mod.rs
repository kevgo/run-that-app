//! the different ways to install an application

pub mod compile_go;
pub mod compile_rust;
mod download_executable;
mod download_packaged_executable;

pub use download_executable::{download_executable, DownloadArgs};
pub use download_packaged_executable::download_packaged_executable;
