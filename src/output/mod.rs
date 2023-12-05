//! Output to the user

mod console;

pub use console::StdErr;

pub trait Output {
    /// indicates whether the given category is active
    fn is_active(&self, category: &str) -> bool;

    /// this output only gets displayed if the respective logging category is enabled
    fn log(&self, category: &str, text: &str);

    /// this output always gets displayed
    fn print(&self, text: &str);

    /// this output always gets displayed
    fn println(&self, text: &str);
}
