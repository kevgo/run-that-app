use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, strings};
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct Snyk {}

const ORG: &str = "snyk";
const REPO: &str = "cli";
const TAG_PREFIX: &str = "v";

impl AppDefinition for Snyk {
  fn name(&self) -> ApplicationName {
    "snyk".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "macos",
      Os::Windows => "win",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "-arm64",
      Cpu::Intel64 => "",
    };
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "",
      Os::Windows => ".exe",
    };
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadExecutable {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/{TAG_PREFIX}{version}/snyk-{os}{cpu}{ext}").into(),
      }],
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, TAG_PREFIX, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, TAG_PREFIX, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("shfmt formats shell programs") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  strings::first_capture(output, r"^(\d+\.\d+\.\d+)$")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::snyk::Snyk;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Snyk {}).run_method(
        &Version::from("1.1304.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/snyk/cli/releases/download/v1.1304.1/snyk-linux-arm64".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (Snyk {}).run_method(
        &Version::from("1.1304.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/snyk/cli/releases/download/v1.1304.1/snyk-linux".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Snyk {}).run_method(
        &Version::from("1.1304.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/snyk/cli/releases/download/v1.1304.1/snyk-macos-arm64".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (Snyk {}).run_method(
        &Version::from("1.1304.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/snyk/cli/releases/download/v1.1304.1/snyk-macos".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Snyk {}).run_method(
        &Version::from("1.1304.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/snyk/cli/releases/download/v1.1304.1/snyk-win.exe".into(),
        }],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("1.1304.1"), Ok("1.1304.1"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
