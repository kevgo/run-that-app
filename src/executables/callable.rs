use crate::error::Result;
use crate::executables::{Executable, UvTool};

/// the different ways to call a loaded app
#[derive(Clone, Debug, PartialEq)]
pub enum Callable {
  /// the app has its own executable that can run directly
  Direct(Executable),
  /// the app is a Python package that runs via "uv tool run"
  UvTool {
    /// the uv executable that runs the tool
    uv: Executable,
    /// the Python package to run
    tool: UvTool,
  },
}

impl Callable {
  /// provides the executable to run in order to call the app
  pub fn executable(&self) -> &Executable {
    match self {
      Callable::Direct(executable) => executable,
      Callable::UvTool { uv, tool: _ } => uv,
    }
  }

  /// provides the arguments that the executable needs before the arguments for the app
  pub fn carrier_args(&self) -> Vec<String> {
    match self {
      Callable::Direct(_) => vec![],
      Callable::UvTool { uv: _, tool } => tool.run_args(),
    }
  }

  /// Provides the app's own executable.
  /// This differs from `executable` for apps that run through a launcher like uv.
  pub fn app_executable(self) -> Result<Executable> {
    match self {
      Callable::Direct(executable) => Ok(executable),
      Callable::UvTool { uv, tool } => Ok(Executable::Binary(tool.executable_path(&uv)?)),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::configuration::Version;
  use crate::executables::{Callable, Executable, UvTool};

  fn uv_tool() -> Callable {
    Callable::UvTool {
      uv: Executable::Binary("yard/uv/uv".into()),
      tool: UvTool {
        package: "python-lsp-server",
        script: "pylsp",
        version: Version::from("1.13.0"),
      },
    }
  }

  mod executable {
    use super::uv_tool;
    use crate::executables::{Callable, Executable};

    #[test]
    fn direct() {
      let callable = Callable::Direct(Executable::ShellScript("yard/npm/bin/npm".into()));
      let have = callable.executable();
      let want = Executable::ShellScript("yard/npm/bin/npm".into());
      assert_eq!(have, &want);
    }

    #[test]
    fn via_uv() {
      let callable = uv_tool();
      let have = callable.executable();
      let want = Executable::Binary("yard/uv/uv".into());
      assert_eq!(have, &want);
    }
  }

  mod args {
    use super::uv_tool;
    use crate::executables::{Callable, Executable};
    use big_s::S;

    #[test]
    fn direct() {
      let callable = Callable::Direct(Executable::Binary("yard/gh/gh".into()));
      let have = callable.carrier_args();
      let want: Vec<String> = vec![];
      assert_eq!(have, want);
    }

    #[test]
    fn via_uv() {
      let have = uv_tool().carrier_args();
      let want = vec![S("tool"), S("run"), S("--from"), S("python-lsp-server@1.13.0"), S("--"), S("pylsp")];
      assert_eq!(have, want);
    }
  }

  mod app_executable {
    use crate::executables::{Callable, Executable};

    #[test]
    fn direct() {
      let callable = Callable::Direct(Executable::Binary("yard/gh/gh".into()));
      let have = callable.app_executable().unwrap();
      let want = Executable::Binary("yard/gh/gh".into());
      assert_eq!(have, want);
    }
  }
}
