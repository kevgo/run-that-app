//! functionality for the `.tool-versions` file

mod app_name;
mod app_versions;
mod config;
mod version;
mod version_for;
mod versions;

pub use app_name::AppName;
pub use app_versions::AppVersions;
pub use config::Config;
pub use version::Version;
pub use version_for::versions_for;
pub use versions::Versions;

pub const FILE_NAME: &str = ".tool-versions";
