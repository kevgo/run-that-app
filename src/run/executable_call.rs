use super::{ExecutableArgs, ExecutablePath};
use std::fmt::{Display, Write};
use std::path::Path;

/// information to call an `App`s executable, as it is defined by the user
#[derive(Clone)]
pub(crate) struct ExecutableCallDefinition {
  pub(crate) executable_path: ExecutablePath,
  pub(crate) args: ExecutableArgs,
}

impl ExecutableCallDefinition {
  pub(crate) fn into_executable_call(self, app_folder: &Path) -> Option<ExecutableCall> {
    match self.args {
      ExecutableArgs::None => Some(ExecutableCall {
        executable_path: self.executable_path,
        args: vec![],
      }),
      ExecutableArgs::OneOfTheseInAppFolder { options } => {
        for option in options {
          let full_path = app_folder.join(option);
          if full_path.exists() {
            return Some(ExecutableCall {
              executable_path: self.executable_path,
              args: vec![full_path.to_string_lossy().to_string()],
            });
          }
        }
        None
      }
    }
  }
}

impl Display for ExecutableCallDefinition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.executable_path.as_str())?;
    f.write_str(&self.args.to_string())?;
    Ok(())
  }
}

/// information to call an app with file paths adjusted
pub(crate) struct ExecutableCall {
  pub(crate) executable_path: ExecutablePath,
  pub(crate) args: Vec<String>,
}

impl ExecutableCall {
  /// provides a printable version of the given executable invocation
  // TODO: move into the upcoming CallSignature
  pub(crate) fn format_call(&self, args: &[String]) -> String {
    let mut result = String::from(self.executable_path.as_str());
    for arg in &self.args {
      result.push(' ');
      result.push_str(arg);
    }
    for arg in args {
      result.push(' ');
      result.push_str(arg);
    }
    result
  }
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
