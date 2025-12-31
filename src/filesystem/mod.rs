//! This module implements accessing the local filesystem.

mod find_global_install;
mod has_extension;
mod make_executable;
mod read_file;
mod save_buffer;

pub(crate) use find_global_install::find_global_install;
pub(crate) use has_extension::has_extension;
pub(crate) use make_executable::make_executable;
pub(crate) use read_file::read_file;
pub(crate) use save_buffer::save_executable;
