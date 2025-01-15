use std::fmt::{Display, Write};

/// Describes how to call an executable.
/// This exists because some executables are called through other executables
/// and we cannot read the properties of Cmd.
pub struct CallSignature<T: Display> {
  pub executable: T,
  /// arguments for the executable
  pub arguments: Vec<String>,
}

impl<T: Display> Display for CallSignature<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.executable.to_string());
    for arg in &self.arguments {
      f.write_char(' ');
      f.write_str(&arg);
    }
    Ok(())
  }
}
