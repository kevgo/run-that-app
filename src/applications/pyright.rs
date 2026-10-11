use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::platform::Platform;
use crate::{Log, strings, subshell};
use const_format::formatcp;

#[derive(Clone)]
pub struct Pyright {}

const ORG: &str = "microsoft";
const REPO: &str = "pyright";

impl AppDefinition for Pyright {
  fn name(&self) -> ApplicationName {
    "pyright".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, _version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::Uv {
      package: "pyright",
      script: "pyright",
    }
  }
  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, &self.tag_format(), log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, &self.tag_format(), log)
  }

  fn analyze_executable(&self, executable: &Executable) -> Result<AnalyzeResult> {
    let output = subshell::capture_output(executable, &["-h"])?;
    if !output.contains("--pythonversion") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match strings::first_version(&subshell::capture_output(executable, &["--version"])?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn tag_format(&self) -> TagFormat {
    TagFormat::Plain
  }
}

#[cfg(test)]
mod tests {

  mod run_method {
    use crate::applications::{AppDefinition, Pyright};
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Pyright {}).run_method(
        &Version::from("1.1.414"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::Uv {
        package: "pyright",
        script: "pyright",
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (Pyright {}).run_method(
        &Version::from("1.1.414"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::Uv {
        package: "pyright",
        script: "pyright",
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Pyright {}).run_method(
        &Version::from("1.1.414"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::Uv {
        package: "pyright",
        script: "pyright",
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (Pyright {}).run_method(
        &Version::from("1.1.414"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::Uv {
        package: "pyright",
        script: "pyright",
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (Pyright {}).run_method(
        &Version::from("1.1.414"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::Uv {
        package: "pyright",
        script: "pyright",
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Pyright {}).run_method(
        &Version::from("1.1.414"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::Uv {
        package: "pyright",
        script: "pyright",
      };
      assert_eq!(have, want);
    }
  }
}
