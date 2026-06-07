//! This module implements the business logic for each top-level command that run-that-app understands.

mod add;
mod applications;
mod available;
mod help;
mod install;
mod install_all;
mod reinstall;
mod run;
mod test;
mod update;
mod version;
mod versions;
mod which;

pub use add::{AddArgs, add};
pub use available::{AvailableArgs, available};
pub(crate) use help::help;
pub use install::{InstallArgs, install};
pub use install_all::install_all;
pub use reinstall::reinstall;
pub use run::{RunArgs, run};
pub use test::{TestArgs, test};
pub use update::{UpdateArgs, update};
pub use version::version;
pub use versions::{VersionsArgs, versions};
pub use which::{WhichArgs, which};
