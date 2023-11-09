//! The area on disk that stores the installed applications.
//! Named after rail yards, i.e. locations where passenger cars of trains are stored, sorted, and repaired.

mod folder_for;
mod install_app;
mod install_if_needed;
mod load_runnable_app;
mod runnable_app;
mod store_app;

pub use folder_for::folder_for;
pub use install_app::install_app;
pub use install_if_needed::load_or_install;
pub use load_runnable_app::load_runnable_app;
pub use runnable_app::RunnableApp;
pub use store_app::store_app;
