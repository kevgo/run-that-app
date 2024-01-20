//! Module systempath provides access to executables that are stored on the system path, outside a yard.

mod find_in_path;
mod system_executable;

pub use system_executable::SystemExecutable;
