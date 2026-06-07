//! This module implements reading the `run-that-app` file.

mod app_versions;
mod file;
mod requested_version;
mod requested_versions;
mod tag_format;
mod version;

pub use app_versions::AppVersions;
pub use file::File;
pub use requested_version::RequestedVersion;
pub use requested_versions::RequestedVersions;
pub use tag_format::TagFormat;
pub use version::Version;

pub const FILE_NAME: &str = "run-that-app";
