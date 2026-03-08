use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, strings};
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct Bun {}

const ORG: &str = "oven-sh";
const REPO: &str = "bun";

impl AppDefinition for Bun {
  fn name(&self) -> ApplicationName {
    "bun".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/oven-sh/bun")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "aarch64",
      Cpu::Intel64 => "x64",
    };
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "windows",
    };
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/bun-v{version}/bun-{os}-{cpu}.zip").into(),
        bin_folder: BinFolder::Subfolder {
          path: format!("bun-{os}-{cpu}").into(),
        },
      }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, "bun-v", log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, "bun-v", log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("Bun is a fast JavaScript runtime") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  strings::first_capture(output, r"(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::bun::Bun;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn macos_arm() {
      let have = (Bun {}).run_method(
        &Version::from("1.3.8"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/oven-sh/bun/releases/download/bun-v1.3.8/bun-darwin-aarch64.zip".into(),
          bin_folder: BinFolder::Subfolder {
            path: "bun-darwin-aarch64".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_arm() {
      let have = (Bun {}).run_method(
        &Version::from("1.3.8"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/oven-sh/bun/releases/download/bun-v1.3.8/bun-linux-aarch64.zip".into(),
          bin_folder: BinFolder::Subfolder {
            path: "bun-linux-aarch64".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Bun {}).run_method(
        &Version::from("1.3.8"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/oven-sh/bun/releases/download/bun-v1.3.8/bun-windows-x64.zip".into(),
          bin_folder: BinFolder::Subfolder {
            path: "bun-windows-x64".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("1.3.10"), Ok("1.3.10"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
