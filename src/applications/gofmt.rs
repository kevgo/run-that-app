use super::go::Go;
use super::{AnalyzeResult, AppDefinition};
use crate::Log;
use crate::configuration::Version;
use crate::executables::{Executable, ExecutableNameUnix, RunMethod};
use crate::platform::Platform;
use crate::prelude::*;

pub(crate) struct Gofmt {}

impl AppDefinition for Gofmt {
  fn name(&self) -> &'static str {
    "gofmt"
  }

  fn homepage(&self) -> &'static str {
    "https://go.dev"
  }

  fn run_method(&self, _version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::OtherAppOtherExecutable {
      app_definition: Box::new(app_to_install()),
      executable_name: ExecutableNameUnix::from("gofmt"),
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    app_to_install().latest_installable_version(log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    app_to_install().installable_versions(amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("report all errors (not just the first 10 on different lines)") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    let go = Go {};
    #[allow(clippy::unwrap_used)]
    let go_path = executable.as_path().parent().unwrap().join(go.executable_filename().as_ref());
    go.analyze_executable(&Executable::from(go_path), log)
  }

  fn clone(&self) -> Box<dyn AppDefinition> {
    Box::new(Self {})
  }
}

fn app_to_install() -> Go {
  Go {}
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::go::Go;
    use crate::applications::gofmt::Gofmt;
    use crate::configuration::Version;
    use crate::executables::{ExecutableNameUnix, RunMethod};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn macos() {
      let have = (Gofmt {}).run_method(
        &Version::from("1.23.4"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::OtherAppOtherExecutable {
        app_definition: Box::new(Go {}),
        executable_name: ExecutableNameUnix::from("gofmt"),
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows() {
      let have = (Gofmt {}).run_method(
        &Version::from("1.23.4"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::OtherAppOtherExecutable {
        app_definition: Box::new(Go {}),
        executable_name: ExecutableNameUnix::from("gofmt"),
      };
      assert_eq!(have, want);
    }
  }
}
