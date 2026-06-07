use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, strings};

#[derive(Clone)]
pub struct Rclone {}

const ORG: &str = "rclone";
const REPO: &str = "rclone";

impl AppDefinition for Rclone {
  fn name(&self) -> ApplicationName {
    "rclone".into()
  }

  fn homepage(&self) -> &'static str {
    "https://rclone.org"
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "osx",
      Os::Windows => "windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "amd64",
    };
    let tag = self.tag_format().format_version(version);
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/{tag}/rclone-v{version}-{os}-{cpu}.zip").into(),
        bin_folder: BinFolder::Subfolder {
          path: format!("rclone-v{version}-{os}-{cpu}").into(),
        },
      }],
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, &self.tag_format(), log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, &self.tag_format(), log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("Rclone syncs files to and from cloud storage providers") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    let output = executable.run_output(&["version"], log)?;
    match extract_version(&output) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn tag_format(&self) -> TagFormat {
    TagFormat::PrefixV
  }
}

fn extract_version(output: &str) -> Result<&str> {
  strings::first_capture(output, r"rclone v(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {

  mod run_method {
    use crate::applications::AppDefinition;
    use crate::applications::rclone::Rclone;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Rclone {}).run_method(
        &Version::from("1.72.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rclone/rclone/releases/download/v1.72.1/rclone-v1.72.1-linux-arm64.zip".into(),
          bin_folder: BinFolder::Subfolder {
            path: "rclone-v1.72.1-linux-arm64".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (Rclone {}).run_method(
        &Version::from("1.72.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rclone/rclone/releases/download/v1.72.1/rclone-v1.72.1-linux-amd64.zip".into(),
          bin_folder: BinFolder::Subfolder {
            path: "rclone-v1.72.1-linux-amd64".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Rclone {}).run_method(
        &Version::from("1.72.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rclone/rclone/releases/download/v1.72.1/rclone-v1.72.1-osx-arm64.zip".into(),
          bin_folder: BinFolder::Subfolder {
            path: "rclone-v1.72.1-osx-arm64".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (Rclone {}).run_method(
        &Version::from("1.72.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rclone/rclone/releases/download/v1.72.1/rclone-v1.72.1-osx-amd64.zip".into(),
          bin_folder: BinFolder::Subfolder {
            path: "rclone-v1.72.1-osx-amd64".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (Rclone {}).run_method(
        &Version::from("1.72.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rclone/rclone/releases/download/v1.72.1/rclone-v1.72.1-windows-arm64.zip".into(),
          bin_folder: BinFolder::Subfolder {
            path: "rclone-v1.72.1-windows-arm64".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Rclone {}).run_method(
        &Version::from("1.72.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rclone/rclone/releases/download/v1.72.1/rclone-v1.72.1-windows-amd64.zip".into(),
          bin_folder: BinFolder::Subfolder {
            path: "rclone-v1.72.1-windows-amd64".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }
  }
}
