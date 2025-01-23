use std::fmt::{Display, Write};
use std::path::{Path, PathBuf};

/// arguments that are required to execute an application itself - these are not arguments provided by the user
#[derive(Clone, Debug, PartialEq)]
pub enum ExecutableArgs {
  /// the executable can be called without any additional arguments
  None,
  /// uses the first of the given options that exists inside the folder that application is installed in
  OneOfTheseInAppFolder { options: Vec<&'static str> },
}

impl ExecutableArgs {
  /// makes the arguments
  pub fn make_absolute(&self, app_folder: &Path) -> Vec<PathBuf> {
    match self {
      ExecutableArgs::None => vec![],
      ExecutableArgs::OneOfTheseInAppFolder { options } => options.iter().map(|option| app_folder.join(option)).collect(),
    }
  }
}

impl Display for ExecutableArgs {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ExecutableArgs::None => f.write_str("no args"),
      ExecutableArgs::OneOfTheseInAppFolder { options } => {
        f.write_str("one of these filesystem entries:")?;
        for option in options {
          f.write_char(' ')?;
          f.write_str(option)?;
        }
        Ok(())
      }
    }
  }
}
