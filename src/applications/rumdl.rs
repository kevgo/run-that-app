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
pub(crate) struct Rumdl {}

const ORG: &str = "rvben";
const REPO: &str = "rumdl";
const TAG_PREFIX: &str = "v";

impl AppDefinition for Rumdl {
  fn name(&self) -> ApplicationName {
    "rumdl".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "aarch64",
      Cpu::Intel64 => "x86_64",
    };
    let os = match platform.os {
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
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/{TAG_PREFIX}{version}/rumdl-{TAG_PREFIX}{version}-{cpu}-{os}.{ext}").into(),
        bin_folder: BinFolder::Root,
      }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, TAG_PREFIX, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, TAG_PREFIX, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("An extremely fast Python linter and code formatter") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  strings::first_capture(output, r"rumdl (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::rumdl::Rumdl;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Rumdl {}).run_method(
        &Version::from("0.1.58"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rvben/rumdl/releases/download/v0.1.58/rumdl-v0.1.58-aarch64-unknown-linux-gnu.tar.gz".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (Rumdl {}).run_method(
        &Version::from("0.1.58"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rvben/rumdl/releases/download/v0.1.58/rumdl-v0.1.58-x86_64-unknown-linux-gnu.tar.gz".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Rumdl {}).run_method(
        &Version::from("0.1.58"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rvben/rumdl/releases/download/v0.1.58/rumdl-v0.1.58-aarch64-apple-darwin.tar.gz".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (Rumdl {}).run_method(
        &Version::from("0.1.58"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rvben/rumdl/releases/download/v0.1.58/rumdl-v0.1.58-x86_64-apple-darwin.tar.gz".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Rumdl {}).run_method(
        &Version::from("0.1.58"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rvben/rumdl/releases/download/v0.1.58/rumdl-v0.1.58-x86_64-pc-windows-msvc.zip".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("rumdl 0.1.58"), Ok("0.1.58"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
