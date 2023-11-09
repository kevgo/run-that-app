//! Output to the user

mod console;

pub use console::Console;

pub trait Output {
    fn log(&self, category: &str, text: &str);
    fn print(&self, text: &str);
    fn println(&self, text: &str);
}
