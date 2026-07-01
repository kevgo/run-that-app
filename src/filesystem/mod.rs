//! This module implements accessing the local filesystem.

mod find_global_install;
mod has_extension;
mod lock;
mod read_file;
mod save_buffer;
mod set_executable_bit;

pub use find_global_install::find_global_install;
pub use has_extension::has_extension;
pub use lock::with_lock;
pub use read_file::read_file;
pub use save_buffer::save_executable;
pub use set_executable_bit::set_executable_bit;
