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
  /// provides a printable version of this ExecutableCall when called with additional arguments
  pub(crate) fn format_with_extra_args(&self, args: &[String]) -> String {
    let mut result = String::from(self.to_string());
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

#[cfg(test)]
mod tests {
  use super::ExecutableCall;
  use crate::run::ExecutablePath;
  use big_s::S;
  use std::path::Path;

  #[test]
  fn to_string() {
    let call = ExecutableCall {
      executable_path: ExecutablePath::from(Path::new("executable")),
      args: vec![S("arg1"), S("arg2")],
    };
    let have = call.to_string();
    let want = S("executable arg1 arg2");
    assert_eq!(have, want);
  }

  #[test]
  fn format_with_extra_args() {
    let call = ExecutableCall {
      executable_path: ExecutablePath::from(Path::new("executable")),
      args: vec![S("arg1"), S("arg2")],
    };
    let have = call.format_with_extra_args(&[S("arg3")]);
    let want = S("executable arg1 arg2 arg3");
    assert_eq!(have, want);
  }
}
