//! This module implements reading the `.app-versions` file.

mod app_versions;
mod file;
mod requested_version;
mod requested_versions;
mod version;

pub(crate) use app_versions::AppVersions;
pub(crate) use file::File;
pub(crate) use requested_version::RequestedVersion;
pub(crate) use requested_versions::RequestedVersions;
pub(crate) use version::Version;

pub(crate) const FILE_NAME: &str = ".app-versions";
