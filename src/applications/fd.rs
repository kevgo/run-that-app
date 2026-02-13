use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, regexp};
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct Fd {}

const ORG: &str = "sharkdp";
const REPO: &str = "fd";

impl AppDefinition for Fd {
  fn name(&self) -> ApplicationName {
    "fd".into()
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
      Os::Windows => "zip",
      Os::Linux | Os::MacOS => "tar.gz",
    };
    RunMethod::ThisApp {
      install_methods: vec![
        Method::DownloadArchive {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/fd-v{version}-{cpu}-{os}.{ext}").into(),
          bin_folder: BinFolder::Root,
        },
        Method::CompileRustSource {
          crate_name: "fd-find",
          bin_folder: BinFolder::Subfolder {
            path: format!("fd-v{version}-{cpu}-{os}").into(),
          },
        },
      ],
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
    if !output.contains("A program to find entries in your filesystem") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"dprint (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use super::super::Fd;
    use crate::applications::AppDefinition;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Fd {}).run_method(
        &Version::from("10.3.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/sharkdp/fd/releases/download/v10.3.0/fd-v10.3.0-aarch64-unknown-linux-gnu.tar.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "fd-find",
            bin_folder: BinFolder::Subfolder {
              path: "fd-v10.3.0-aarch64-unknown-linux-gnu".into(),
            },
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_x86() {
      let have = (Fd {}).run_method(
        &Version::from("10.3.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/sharkdp/fd/releases/download/v10.3.0/fd-v10.3.0-x86_64-unknown-linux-gnu.tar.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "fd-find",
            bin_folder: BinFolder::Subfolder {
              path: "fd-v10.3.0-x86_64-unknown-linux-gnu".into(),
            },
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Fd {}).run_method(
        &Version::from("10.3.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/sharkdp/fd/releases/download/v10.3.0/fd-v10.3.0-aarch64-apple-darwin.tar.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "fd-find",
            bin_folder: BinFolder::Subfolder {
              path: "fd-v10.3.0-aarch64-apple-darwin".into(),
            },
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_x86() {
      let have = (Fd {}).run_method(
        &Version::from("10.3.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/sharkdp/fd/releases/download/v10.3.0/fd-v10.3.0-x86_64-apple-darwin.tar.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "fd-find",
            bin_folder: BinFolder::Subfolder {
              path: "fd-v10.3.0-x86_64-apple-darwin".into(),
            },
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (Fd {}).run_method(
        &Version::from("10.3.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/sharkdp/fd/releases/download/v10.3.0/fd-v10.3.0-aarch64-pc-windows-msvc.zip".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "fd-find",
            bin_folder: BinFolder::Subfolder {
              path: "fd-v10.3.0-aarch64-pc-windows-msvc".into(),
            },
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_x86() {
      let have = (Fd {}).run_method(
        &Version::from("10.3.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/sharkdp/fd/releases/download/v10.3.0/fd-v10.3.0-x86_64-pc-windows-msvc.zip".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "fd-find",
            bin_folder: BinFolder::Subfolder {
              path: "fd-v10.3.0-x86_64-pc-windows-msvc".into(),
            },
          },
        ],
      };
      assert_eq!(have, want);
    }
  }
}
