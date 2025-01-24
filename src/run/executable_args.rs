use crate::installation::BinFolder;
use crate::prelude::*;
use std::fmt::{Display, Write};
use std::path::Path;

/// Arguments that are required to execute an application itself - these are not arguments provided by the user.
/// Example: running npm happens as "node npm.js", "npm.js" is the executable arg.
#[derive(Clone, Debug, PartialEq)]
pub enum ExecutableArgs {
  /// the executable is called without any additional arguments
  None,
  /// uses the first of the given options that exists inside the folder that application is installed in
  OneOfTheseInAppFolder { options: Vec<&'static str> },
}

impl ExecutableArgs {
  /// provides the argument to use, adjusted to a callable format
  pub fn locate(&self, app_folder: &Path, bin_folder: BinFolder) -> Result<Vec<String>> {
    match self {
      ExecutableArgs::None => Ok(vec![]),
      ExecutableArgs::OneOfTheseInAppFolder { options } => {
        let bin_folder_paths = bin_folder.possible_paths(app_folder);
        for option in options {
          for bin_folder_path in &bin_folder_paths {
            let absolute_path = bin_folder_path.join(option);
            if absolute_path.exists() {
              return Ok(vec![absolute_path.to_string_lossy().to_string()]);
            }
          }
        }
        Err(UserError::CannotFindExecutable)
      }
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
