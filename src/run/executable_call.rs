use super::ExecutablePath;
use std::fmt::{Display, Write};

/// a way to call an executable
pub struct ExecutableCall {
  /// the executable to call
  pub executable: ExecutablePath,
  /// arguments that are part of running the executable itself, not arguments provided by the user
  pub args: Vec<&'static str>,
}

impl Display for ExecutableCall {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.executable.as_str())?;
    for arg in &self.args {
      f.write_char(' ')?;
      f.write_str(arg)?;
    }
    Ok(())
  }
}
