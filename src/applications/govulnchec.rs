use super::{AnalyzeResult, AppDefinition};
use crate::configuration::Version;
use crate::executables::{self, Executable, RunMethod};
use crate::installation::Method;
use crate::platform::Platform;
use crate::prelude::*;
use crate::Log;

pub(crate) struct Govulncheck {}

impl AppDefinition for Govulncheck {
  fn name(&self) -> &'static str {
    "govulncheck"
  }

  fn homepage(&self) -> &'static str {
    "https://pkg.go.dev/golang.org/x/vuln/cmd/govulncheck"
  }

  fn run_method(&self, version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::ThisApp {
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

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("Govulncheck reports known vulnerabilities in dependencies") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // govulncheck does not display the version of the installed executable
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn clone(&self) -> Box<dyn AppDefinition> {
    Box::new(Self {})
  }

  fn executable_filename(&self) -> executables::ExecutableNameUnix {
    executables::ExecutableNameUnix::from(self.name())
  }

  fn additional_executables(&self) -> Vec<executables::ExecutableNameUnix> {
    std::vec![]
  }

  fn allowed_versions(&self) -> Result<semver::VersionReq> {
    Ok(semver::VersionReq::STAR)
  }

  fn app_name(&self) -> super::ApplicationName {
    super::ApplicationName(self.name())
  }

  fn carrier(&self, version: &Version, platform: Platform) -> (Box<dyn AppDefinition>, executables::ExecutableNameUnix, executables::ExecutableArgs) {
    match self.run_method(version, platform) {
      RunMethod::ThisApp { install_methods: _ } => (self.clone(), self.executable_filename(), executables::ExecutableArgs::None),
      RunMethod::OtherAppOtherExecutable {
        app_definition,
        executable_name,
      } => (app_definition.clone(), executable_name, executables::ExecutableArgs::None),
      RunMethod::OtherAppDefaultExecutable { app_definition, args } => (app_definition.clone(), app_definition.executable_filename(), args),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::executables::{self, RunMethod};

  #[test]
  fn install_methods() {
    use crate::applications::govulnchec::Govulncheck;
    use crate::applications::AppDefinition;
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
    let want = RunMethod::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: S("golang.org/x/vuln/cmd/govulncheck@v1.1.4"),
      }],
    };
    assert_eq!(have, want);
  }
}
