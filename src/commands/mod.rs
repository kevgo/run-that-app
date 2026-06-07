//! This module implements the business logic for each top-level command that run-that-app understands.

pub(crate) mod add;
pub(crate) mod applications;
pub(crate) mod available;
mod help;
pub(crate) mod install;
pub(crate) mod install_all;
pub(crate) mod reinstall;
pub(crate) mod run;
pub(crate) mod test;
pub(crate) mod update;
mod version;
pub(crate) mod versions;
pub(crate) mod which;

pub use add::add;
pub use available::available;
pub(crate) use help::help;
pub use install::install;
pub use install_all::install_all;
pub use reinstall::reinstall;
pub use run::run;
pub(crate) use test::test;
pub use update::update;
pub use version::version;
pub use versions::versions;
pub use which::which;
