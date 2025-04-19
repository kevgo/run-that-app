use super::{AnalyzeResult, AppDefinition};
use crate::configuration::Version;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::{Log, executables, regexp};
use const_format::formatcp;

pub(crate) struct RipGrep {}

const ORG: &str = "BurntSushi";
const REPO: &str = "ripgrep";

impl AppDefinition for RipGrep {
  fn name(&self) -> &'static str {
    "ripgrep"
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn executable_filename(&self) -> executables::ExecutableNameUnix {
    executables::ExecutableNameUnix::from("rg")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "aarch64",
      Cpu::Intel64 => "x86_64",
    };
    let os = match platform.os {
      Os::Linux if platform.cpu == Cpu::Intel64 => "unknown-linux-musl",
      Os::Linux => "unknown-linux-gnu",
      Os::MacOS => "apple-darwin",
      Os::Windows => "pc-windows-msvc",
    };
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "tar.gz",
      Os::Windows => "zip",
    };
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/{version}/ripgrep-{version}-{cpu}-{os}.{ext}"),
        bin_folder: BinFolder::Subfolder {
          path: format!("ripgrep-{version}-{cpu}-{os}"),
        },
      }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("ripgrep") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&output) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn clone(&self) -> Box<dyn AppDefinition> {
    Box::new(Self {})
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"ripgrep (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::ripgrep::RipGrep;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn macos_arm() {
      let have = (RipGrep {}).run_method(
        &Version::from("14.1.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/BurntSushi/ripgrep/releases/download/14.1.1/ripgrep-14.1.1-aarch64-apple-darwin.tar.gz"),
          bin_folder: BinFolder::Subfolder {
            path: S("ripgrep-14.1.1-aarch64-apple-darwin"),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_arm() {
      let have = (RipGrep {}).run_method(
        &Version::from("14.1.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/BurntSushi/ripgrep/releases/download/14.1.1/ripgrep-14.1.1-aarch64-unknown-linux-gnu.tar.gz"),
          bin_folder: BinFolder::Subfolder {
            path: S("ripgrep-14.1.1-aarch64-unknown-linux-gnu"),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (RipGrep {}).run_method(
        &Version::from("14.1.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/BurntSushi/ripgrep/releases/download/14.1.1/ripgrep-14.1.1-x86_64-unknown-linux-musl.tar.gz"),
          bin_folder: BinFolder::Subfolder {
            path: S("ripgrep-14.1.1-x86_64-unknown-linux-musl"),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (RipGrep {}).run_method(
        &Version::from("14.1.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/BurntSushi/ripgrep/releases/download/14.1.1/ripgrep-14.1.1-x86_64-pc-windows-msvc.zip"),
          bin_folder: BinFolder::Subfolder {
            path: S("ripgrep-14.1.1-x86_64-pc-windows-msvc"),
          },
        }],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("ripgrep 14.1.1"), Ok("14.1.1"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
