use crate::CommandInfo;
use crate::configuration::Version;
use crate::error::{Result, UserError};
use crate::executables::Executable;
use big_s::S;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Python code that prints the full path of the executable with the given name in the PATH
const LOCATE_EXECUTABLE: &str = "import shutil, sys; print(shutil.which(sys.argv[1]) or '')";

/// a Python package that runs via "uv tool run"
#[derive(Clone, Debug, PartialEq)]
pub struct UvTool {
  /// name of the Python package that provides the script
  pub package: &'static str,

  /// unix name of the script to run
  pub script: &'static str,

  /// version of the Python package to run
  pub version: Version,
}

impl UvTool {
  /// provides the arguments to call uv with to run this tool
  pub fn run_args(&self) -> Vec<String> {
    self.uv_args(&[self.script])
  }

  /// Provides the path of the executable that "uv tool run" executes for this tool.
  /// This asks uv for the path, so that it remains correct when uv changes how it organizes its cache.
  pub fn executable_path(&self, uv: &Executable) -> Result<PathBuf> {
    // "uv tool run" puts the folder with the tool's executables at the start of the PATH,
    // so the Python interpreter in the tool's environment finds the tool's executable there.
    // The "--quiet" flag ensures that the output of the Python code is the only output.
    let mut args = vec![S("--quiet")];
    args.extend(self.uv_args(&["python", "-c", LOCATE_EXECUTABLE, self.script]));
    let call = CommandInfo {
      executable: uv.as_path().to_path_buf(),
      args: Some(args.clone()),
      env_path: None,
    };
    let output = match Command::new(uv).args(&args).stderr(Stdio::inherit()).output() {
      Ok(output) => output,
      Err(err) => return Err(UserError::CannotExecuteBinary { call, reason: err.to_string() }),
    };
    if !output.status.success() {
      return Err(UserError::CannotExecuteBinary {
        call,
        reason: output.status.to_string(),
      });
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let path = stdout.trim();
    if path.is_empty() {
      return Err(UserError::CannotExecuteBinary {
        call,
        reason: format!("executable {} not found in the environment of {}@{}", self.script, self.package, self.version),
      });
    }
    Ok(PathBuf::from(path))
  }

  /// provides the arguments to call uv with to run the given command in the environment of this tool
  fn uv_args(&self, command: &[&str]) -> Vec<String> {
    let mut result = vec![S("tool"), S("run"), S("--from"), format!("{}@{}", self.package, self.version), S("--")];
    result.extend(command.iter().map(ToString::to_string));
    result
  }
}

#[cfg(test)]
mod tests {

  mod run_args {
    use crate::configuration::Version;
    use crate::executables::UvTool;
    use big_s::S;

    #[test]
    fn package_provides_same_script() {
      let tool = UvTool {
        package: "pyright",
        script: "pyright",
        version: Version::from("1.1.414"),
      };
      let have = tool.run_args();
      let want = vec![S("tool"), S("run"), S("--from"), S("pyright@1.1.414"), S("--"), S("pyright")];
      assert_eq!(have, want);
    }

    #[test]
    fn package_provides_different_script() {
      let tool = UvTool {
        package: "python-lsp-server",
        script: "pylsp",
        version: Version::from("1.13.0"),
      };
      let have = tool.run_args();
      let want = vec![S("tool"), S("run"), S("--from"), S("python-lsp-server@1.13.0"), S("--"), S("pylsp")];
      assert_eq!(have, want);
    }
  }
}
