use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::platform::Platform;
use crate::{Log, strings};
use const_format::formatcp;

#[derive(Clone)]
pub struct MarkdownLint {}

const ORG: &str = "igorshubovych";
const REPO: &str = "markdownlint-cli";

impl AppDefinition for MarkdownLint {
  fn name(&self) -> ApplicationName {
    "markdownlint".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, _version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::NodeJS { package: "markdownlint-cli" }
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
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn tag_format(&self) -> TagFormat {
    TagFormat::PrefixV
  }
}

fn extract_version(output: &str) -> Result<&str> {
  strings::first_capture(output, r"(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {

  mod run_method {
    use crate::applications::{AppDefinition, MarkdownLint};
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (MarkdownLint {}).run_method(
        &Version::from("0.49.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::NodeJS { package: "markdownlint-cli" };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (MarkdownLint {}).run_method(
        &Version::from("0.49.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::NodeJS { package: "markdownlint-cli" };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (MarkdownLint {}).run_method(
        &Version::from("0.49.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::NodeJS { package: "markdownlint-cli" };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (MarkdownLint {}).run_method(
        &Version::from("0.49.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::NodeJS { package: "markdownlint-cli" };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (MarkdownLint {}).run_method(
        &Version::from("0.49.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::NodeJS { package: "markdownlint-cli" };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (MarkdownLint {}).run_method(
        &Version::from("0.49.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::NodeJS { package: "markdownlint-cli" };
      assert_eq!(have, want);
    }
  }
}
