use std::fmt::{Display, Write};

/// Describes how to call an executable.
/// This exists because some executables are called through other executables
/// and we cannot read the properties of Cmd.
pub struct CallSignature {
  pub executable_name: String,
  /// arguments for the executable
  pub arguments: Vec<String>,
}

impl Display for CallSignature {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.executable_name);
    for arg in &self.arguments {
      f.write_char(' ');
      f.write_str(&arg);
    }
    Ok(())
  }
}
