//! This module implements the business logic for each top-level command that run-that-app understands.

pub mod add;
pub mod applications;
pub mod available;
mod help;
pub mod install;
pub mod install_all;
pub mod reinstall;
pub mod run;
pub mod test;
pub mod update;
mod version;
pub mod versions;
pub mod which;

pub use add::add;
pub use available::available;
pub use help::help;
pub use install::install;
pub use install_all::install_all;
pub use reinstall::reinstall;
pub use run::run;
pub use test::test;
pub use update::update;
pub use version::version;
pub use versions::versions;
pub use which::which;
