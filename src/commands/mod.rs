//! This module implements the business logic for each top-level command that run-that-app understands.

pub(crate) mod applications;
pub(crate) mod available;
mod help;
pub(crate) mod run;
mod setup;
pub(crate) mod test;
pub(crate) mod update;
mod version;
pub(crate) mod versions;
pub(crate) mod which;

pub(crate) use available::available;
pub(crate) use help::help;
pub(crate) use run::run;
pub(crate) use setup::setup;
pub(crate) use test::test;
pub(crate) use update::update;
pub(crate) use version::version;
pub(crate) use versions::versions;
pub(crate) use which::which;
