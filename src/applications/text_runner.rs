use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::platform::Platform;
use crate::{Log, strings};
use const_format::formatcp;

#[derive(Clone)]
pub struct TextRunner {}

const ORG: &str = "kevgo";
const REPO: &str = "text-runner";

impl AppDefinition for TextRunner {
  fn name(&self) -> ApplicationName {
    "text-runner".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, _version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::NodeJS { package: "text-runner" }
  }
  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, &self.tag_format(), log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, &self.tag_format(), log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("MarkdownLint Command Line Interface") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match strings::first_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn tag_format(&self) -> TagFormat {
    TagFormat::PrefixV
  }
}

#[cfg(test)]
mod tests {

  mod run_method {
    use crate::applications::{AppDefinition, TextRunner};
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (TextRunner {}).run_method(
        &Version::from("7.1.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::NodeJS { package: "text-runner" };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (TextRunner {}).run_method(
        &Version::from("7.1.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::NodeJS { package: "text-runner" };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (TextRunner {}).run_method(
        &Version::from("7.1.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::NodeJS { package: "text-runner" };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (TextRunner {}).run_method(
        &Version::from("7.1.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::NodeJS { package: "text-runner" };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (TextRunner {}).run_method(
        &Version::from("7.1.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::NodeJS { package: "text-runner" };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (TextRunner {}).run_method(
        &Version::from("7.1.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::NodeJS { package: "text-runner" };
      assert_eq!(have, want);
    }
  }
}
