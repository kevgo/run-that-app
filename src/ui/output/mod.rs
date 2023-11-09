mod console;

pub use console::ConsoleOutput;

pub trait Output {
    fn log(&self, category: &str, text: &str);
    fn print(&self, text: &str);
    fn println(&self, text: &str);
    fn flush(&self);
}
