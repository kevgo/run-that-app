use super::{AnalyzeResult, AppDefinition};
use crate::Log;
use crate::applications::ApplicationName;
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::pkg_go_dev;
use crate::installation::Method;
use crate::platform::Platform;

#[derive(Clone)]
pub(crate) struct Deadcode {}

impl AppDefinition for Deadcode {
  fn name(&self) -> ApplicationName {
    "deadcode".into()
  }

  fn homepage(&self) -> &'static str {
    "https://pkg.go.dev/golang.org/x/tools/cmd/deadcode"
  }

  fn run_method(&self, version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: format!("golang.org/x/tools/cmd/deadcode@v{version}"),
      }],
    }
  }

  fn latest_installable_version(&self, _log: Log) -> Result<Version> {
    pkg_go_dev::latest("golang.org/x/tools")
  }

  fn installable_versions(&self, amount: usize, _log: Log) -> Result<Vec<Version>> {
    pkg_go_dev::versions("golang.org/x/tools", amount)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("The deadcode command reports unreachable functions in Go programs") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // as of 0.16.1 deadcode does not display the version of the installed executable
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

#[cfg(test)]
mod tests {
  use crate::applications::deadcode::Deadcode;
  use crate::executables::RunMethod;

  #[test]
  fn install_methods() {
    use crate::applications::AppDefinition;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    let have = (Deadcode {}).run_method(
      &Version::from("0.16.1"),
      Platform {
        os: Os::Linux,
        cpu: Cpu::Arm64,
      },
    );
    let want = RunMethod::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: S("golang.org/x/tools/cmd/deadcode@v0.16.1"),
      }],
    };
    assert_eq!(have, want);
  }
}
