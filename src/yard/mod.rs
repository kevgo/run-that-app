//! The area on disk that stores the installed applications.
//! Named after rail yards, i.e. locations where passenger cars of trains are stored, sorted, and repaired.

mod production_instance;
mod runnable_app;
mod yard;

pub use production_instance::production;
pub use runnable_app::RunnableApp;
pub use yard::Yard;
