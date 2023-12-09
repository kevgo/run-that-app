//! the different ways to install an application

pub mod compile_go;
pub mod compile_rust;
pub mod download_executable;
pub mod download_packaged_executable;

pub use download_executable::download_executable;
pub use download_packaged_executable::download_packaged_executable;
