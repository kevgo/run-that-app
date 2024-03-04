//! functionality for the `.tool-versions` file

mod app_name;
mod app_version;
mod app_versions;
mod config;
mod create;
mod save;
mod version;
mod versions;

pub use app_name::AppName;
pub use app_version::AppVersion;
pub use app_versions::AppVersions;
pub use config::Config;
pub use create::create;
pub use save::save;
pub use version::Version;
pub use versions::Versions;

pub const FILE_NAME: &str = ".tool-versions";
