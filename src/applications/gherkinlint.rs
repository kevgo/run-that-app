use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::Log;
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::platform::Platform;
use const_format::formatcp;

#[derive(Clone)]
pub struct GherkinLint {}

const ORG: &str = "gherkin-lint";
const REPO: &str = "gherkin-lint";

impl AppDefinition for GherkinLint {
  fn name(&self) -> ApplicationName {
    "gherkin-lint".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, _version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::NodeJS {
      package_name: "gherkin-lint".into(),
      executable_path: "node_modules/.bin/gherkin-lint".into(),
    }
  }
  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, &self.tag_format(), log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, &self.tag_format(), log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains(".gherkin-lintrc") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // gherkin-lint has no version command
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn tag_format(&self) -> TagFormat {
    TagFormat::Plain
  }
}

#[cfg(test)]
mod tests {

  mod run_method {
    use crate::applications::{AppDefinition, GherkinLint};
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (GherkinLint {}).run_method(
        &Version::from("4.2.4"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::NodeJS {
        package_name: "gherkin-lint".into(),
        executable_path: "node_modules/.bin/gherkin-lint".into(),
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (GherkinLint {}).run_method(
        &Version::from("4.2.4"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::NodeJS {
        package_name: "gherkin-lint".into(),
        executable_path: "node_modules/.bin/gherkin-lint".into(),
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (GherkinLint {}).run_method(
        &Version::from("4.2.4"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::NodeJS {
        package_name: "gherkin-lint".into(),
        executable_path: "node_modules/.bin/gherkin-lint".into(),
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (GherkinLint {}).run_method(
        &Version::from("4.2.4"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::NodeJS {
        package_name: "gherkin-lint".into(),
        executable_path: "node_modules/.bin/gherkin-lint".into(),
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (GherkinLint {}).run_method(
        &Version::from("3.4.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::NodeJS {
        package_name: "gherkin-lint".into(),
        executable_path: "node_modules/.bin/gherkin-lint".into(),
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (GherkinLint {}).run_method(
        &Version::from("4.2.4"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::NodeJS {
        package_name: "gherkin-lint".into(),
        executable_path: "node_modules/.bin/gherkin-lint".into(),
      };
      assert_eq!(have, want);
    }
  }
}
