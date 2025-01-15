//! functionality for the `.tool-versions` file

mod app_versions;
mod application_name;
mod file;
mod requested_version;
mod requested_versions;
mod version;

pub use app_versions::AppVersions;
pub use application_name::ApplicationName;
pub use file::File;
pub use requested_version::RequestedVersion;
pub use requested_versions::RequestedVersions;
pub use version::Version;

pub const FILE_NAME: &str = ".tool-versions";
