use super::ExecutablePath;
use std::fmt::{Display, Write};

/// all the information needed to call an `App`s executable
pub struct ExecutableCall {
  /// the executable to call
  pub executable_path: ExecutablePath,
  /// arguments that are part of running the executable itself, not arguments provided by the user
  pub args: Vec<String>,
}

impl Display for ExecutableCall {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.executable_path.as_str())?;
    for arg in &self.args {
      f.write_char(' ')?;
      f.write_str(arg)?;
    }
    Ok(())
  }
}
