use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, strings, subshell};
use const_format::formatcp;

#[derive(Clone)]
pub struct Hadolint {}

const ORG: &str = "hadolint";
const REPO: &str = "hadolint";

impl AppDefinition for Hadolint {
  fn name(&self) -> ApplicationName {
    "hadolint".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "macos",
      Os::Windows => "windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "x86_64",
    };
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "",
      Os::Windows => ".exe",
    };
    let tag = self.tag_format().format_version(version);
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadExecutable {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/{tag}/hadolint-{os}-{cpu}{ext}").into(),
      }],
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, &self.tag_format(), log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, &self.tag_format(), log)
  }

  fn analyze_executable(&self, executable: &Executable) -> Result<AnalyzeResult> {
    let output = subshell::capture_output(executable, &["-h"])?;
    if !output.contains("Dockerfile Linter written in Haskell") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match strings::first_version(&subshell::capture_output(executable, &["--version"])?) {
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
    use crate::applications::AppDefinition;
    use crate::applications::hadolint::Hadolint;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Hadolint {}).run_method(
        &Version::from("2.15.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/hadolint/hadolint/releases/download/v2.15.1/hadolint-linux-arm64".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (Hadolint {}).run_method(
        &Version::from("2.15.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/hadolint/hadolint/releases/download/v2.15.1/hadolint-linux-x86_64".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Hadolint {}).run_method(
        &Version::from("2.15.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/hadolint/hadolint/releases/download/v2.15.1/hadolint-macos-arm64".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (Hadolint {}).run_method(
        &Version::from("2.15.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/hadolint/hadolint/releases/download/v2.15.1/hadolint-macos-x86_64".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (Hadolint {}).run_method(
        &Version::from("2.15.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/hadolint/hadolint/releases/download/v2.15.1/hadolint-windows-arm64.exe".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Hadolint {}).run_method(
        &Version::from("2.15.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/hadolint/hadolint/releases/download/v2.15.1/hadolint-windows-x86_64.exe".into(),
        }],
      };
      assert_eq!(have, want);
    }
  }
}
