use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, strings};
use std::path::MAIN_SEPARATOR;

#[derive(Clone)]
pub struct NodeJS {}

pub const ORG: &str = "nodejs";
pub const REPO: &str = "node";

impl AppDefinition for NodeJS {
  fn name(&self) -> ApplicationName {
    "node".into()
  }

  fn homepage(&self) -> &'static str {
    "https://nodejs.org"
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let os = os_text(platform.os);
    let cpu = cpu_text(platform.cpu);
    let ext = ext_text(platform.os);
    let tag = self.tag_format().format_version(version);
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://nodejs.org/dist/v{version}/node-{tag}-{os}-{cpu}.{ext}").into(),
        bin_folder: BinFolder::RootOrSubfolders {
          options: vec![
            format!("node-v{version}-{os}-{cpu}").into(),
            format!("node-v{version}-{os}-{cpu}{MAIN_SEPARATOR}bin").into(),
          ],
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
    if !output.contains("Documentation can be found at https://nodejs.org") {
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

pub fn cpu_text(cpu: Cpu) -> &'static str {
  match cpu {
    Cpu::Arm64 => "arm64",
    Cpu::Intel64 => "x64",
  }
}

fn ext_text(os: Os) -> &'static str {
  match os {
    Os::Linux => "tar.xz",
    Os::MacOS => "tar.gz",
    Os::Windows => "zip",
  }
}

pub fn os_text(os: Os) -> &'static str {
  match os {
    Os::Linux => "linux",
    Os::MacOS => "darwin",
    Os::Windows => "win",
  }
}

#[cfg(test)]
mod tests {

  mod run_method {
    use crate::applications::AppDefinition;
    use crate::applications::nodejs::NodeJS;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    #[cfg(not(windows))]
    fn linux_arm() {
      let have = (NodeJS {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://nodejs.org/dist/v20.10.0/node-v20.10.0-linux-arm64.tar.xz".into(),
          bin_folder: BinFolder::RootOrSubfolders {
            options: vec!["node-v20.10.0-linux-arm64".into(), "node-v20.10.0-linux-arm64/bin".into()],
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn linux_intel() {
      let have = (NodeJS {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://nodejs.org/dist/v20.10.0/node-v20.10.0-linux-x64.tar.xz".into(),
          bin_folder: BinFolder::RootOrSubfolders {
            options: vec!["node-v20.10.0-linux-x64".into(), "node-v20.10.0-linux-x64/bin".into()],
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn macos_arm() {
      let have = (NodeJS {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://nodejs.org/dist/v20.10.0/node-v20.10.0-darwin-arm64.tar.gz".into(),
          bin_folder: BinFolder::RootOrSubfolders {
            options: vec!["node-v20.10.0-darwin-arm64".into(), "node-v20.10.0-darwin-arm64/bin".into()],
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn macos_intel() {
      let have = (NodeJS {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://nodejs.org/dist/v20.10.0/node-v20.10.0-darwin-x64.tar.gz".into(),
          bin_folder: BinFolder::RootOrSubfolders {
            options: vec!["node-v20.10.0-darwin-x64".into(), "node-v20.10.0-darwin-x64/bin".into()],
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_arm() {
      let have = (NodeJS {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://nodejs.org/dist/v20.10.0/node-v20.10.0-win-arm64.zip".into(),
          bin_folder: BinFolder::RootOrSubfolders {
            options: vec!["node-v20.10.0-win-arm64".into(), r"node-v20.10.0-win-arm64\bin".into()],
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_intel() {
      let have = (NodeJS {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://nodejs.org/dist/v20.10.0/node-v20.10.0-win-x64.zip".into(),
          bin_folder: BinFolder::RootOrSubfolders {
            options: vec!["node-v20.10.0-win-x64".into(), r"node-v20.10.0-win-x64\bin".into()],
          },
        }],
      };
      assert_eq!(have, want);
    }
  }
}
