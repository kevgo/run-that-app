use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::Log;
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::Platform;
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct Exhaustruct {}

const ORG: &str = "GaijinEntertainment";
const REPO: &str = "go-exhaustruct";

impl AppDefinition for Exhaustruct {
  fn name(&self) -> ApplicationName {
    "exhaustruct".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn run_method(&self, version: &Version, _platform: Platform) -> RunMethod {
    let major_version = version.major_version().unwrap_or(3);
    let import_path = if major_version >= 4 {
      format!("dev.gaijin.team/go/exhaustruct/v{major_version}/cmd/exhaustruct@v{version}")
    } else {
      format!("github.com/{ORG}/{REPO}/v{major_version}/cmd/exhaustruct@v{version}")
    };
    RunMethod::ThisApp {
      install_methods: vec![Method::CompileGoSource { import_path }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("exhaustruct: Checks if all structure fields are initialized") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

#[cfg(test)]
mod tests {
  use crate::applications::exhaustruct::Exhaustruct;
  use crate::executables::RunMethod;

  #[test]
  fn install_methods() {
    use crate::applications::AppDefinition;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    let have = (Exhaustruct {}).run_method(
      &Version::from("3.3.0"),
      Platform {
        os: Os::Linux,
        cpu: Cpu::Arm64,
      },
    );
    let want = RunMethod::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: S("github.com/GaijinEntertainment/go-exhaustruct/v3/cmd/exhaustruct@v3.3.0"),
      }],
    };
    assert_eq!(have, want);
  }
}
