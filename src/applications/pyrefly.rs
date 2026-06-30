use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, strings};

#[derive(Clone)]
pub struct Pyrefly {}

const ORG: &str = "facebook";
const REPO: &str = "pyrefly";

impl AppDefinition for Pyrefly {
  fn name(&self) -> ApplicationName {
    "pyrefly".into()
  }

  fn homepage(&self) -> &'static str {
    "https://pyrefly.org"
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "x86_64",
    };
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "macos",
      Os::Windows => "windows",
    };
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "tar.gz",
      Os::Windows => "zip",
    };
    let tag = self.tag_format().format_version(version);
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/{tag}/pyrefly-{os}-{cpu}.{ext}").into(),
        bin_folder: BinFolder::Root,
      }],
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
    if !output.contains("A fast Python type checker") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match strings::capture_version(&executable.run_output(&["--version"], log)?) {
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
    use crate::applications::AppDefinition;
    use crate::applications::pyrefly::Pyrefly;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Pyrefly {}).run_method(
        &Version::from("0.57.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/facebook/pyrefly/releases/download/0.57.1/pyrefly-linux-arm64.tar.gz".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (Pyrefly {}).run_method(
        &Version::from("0.57.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/facebook/pyrefly/releases/download/0.57.1/pyrefly-linux-x86_64.tar.gz".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Pyrefly {}).run_method(
        &Version::from("0.57.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/facebook/pyrefly/releases/download/0.57.1/pyrefly-macos-arm64.tar.gz".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (Pyrefly {}).run_method(
        &Version::from("0.57.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/facebook/pyrefly/releases/download/0.57.1/pyrefly-macos-x86_64.tar.gz".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (Pyrefly {}).run_method(
        &Version::from("0.57.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/facebook/pyrefly/releases/download/0.57.1/pyrefly-windows-arm64.zip".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Pyrefly {}).run_method(
        &Version::from("0.57.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/facebook/pyrefly/releases/download/0.57.1/pyrefly-windows-x86_64.zip".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }
  }
}
