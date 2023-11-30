//! Output to the user

mod console;

pub use console::StdErr;

pub trait Output {
  /// this output only gets displayed if the respective logging category is enabled
  fn log(&self, category: &str, text: &str);

  /// this output always gets displayed
  fn print(&self, text: &str);

  /// this output always gets displayed
  fn println(&self, text: &str);
}
