use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::installation::Method;
use crate::platform::Platform;
use crate::prelude::*;
use crate::run::{self, ExecutablePath};
use crate::Log;

pub struct Govulncheck {}

impl App for Govulncheck {
  fn name(&self) -> &'static str {
    "govulncheck"
  }

  fn homepage(&self) -> &'static str {
    "https://pkg.go.dev/golang.org/x/vuln/cmd/govulncheck"
  }

  fn run_method(&self, version: &Version, _platform: Platform) -> run::Method {
    run::Method::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: format!("golang.org/x/vuln/cmd/govulncheck@v{version}"),
      }],
    }
  }

  fn latest_installable_version(&self, _log: Log) -> Result<Version> {
    // TODO: remove this file once govulncheck is integrated into golangci-lint
    Ok(Version::from("1.1.4"))
  }

  fn installable_versions(&self, _amount: usize, _log: Log) -> Result<Vec<Version>> {
    Ok(vec![Version::from("1.1.4"), Version::from("1.1.3")])
  }

  fn analyze_executable(&self, executable: &ExecutablePath, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Govulncheck reports known vulnerabilities in dependencies") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // govulncheck does not display the version of the installed executable
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn clone(&self) -> Box<dyn App> {
    Box::new(Self {})
  }
}

#[cfg(test)]
mod tests {
  use crate::run;

  #[test]
  fn install_methods() {
    use crate::applications::govulnchec::Govulncheck;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    let have = (Govulncheck {}).run_method(
      &Version::from("1.1.4"),
      Platform {
        os: Os::MacOS,
        cpu: Cpu::Arm64,
      },
    );
    let want = run::Method::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: S("golang.org/x/vuln/cmd/govulncheck@v1.1.4"),
      }],
    };
    assert_eq!(have, want);
  }
}
