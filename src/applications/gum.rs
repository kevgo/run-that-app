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
pub(crate) struct Gum {}

const ORG: &str = "charmbracelet";
const REPO: &str = "gum";
const TAG_PREFIX: &str = "v";

impl AppDefinition for Gum {
  fn name(&self) -> ApplicationName {
    "gum".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let os = match platform.os {
      Os::Linux => "Linux",
      Os::MacOS => "Darwin",
      Os::Windows => "Windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "x86_64",
    };
    let ext = match platform.os {
      Os::Windows => "zip",
      Os::Linux | Os::MacOS => "tar.gz",
    };
    RunMethod::ThisApp {
      install_methods: vec![
        Method::DownloadArchive {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/{TAG_PREFIX}{version}/gum_{version}_{os}_{cpu}.{ext}").into(),
          bin_folder: BinFolder::Subfolder {
            path: format!("gum_{version}_{os}_{cpu}").into(),
          },
        },
        Method::CompileGoSource {
          import_path: format!("github.com/{ORG}/{REPO}@latest"),
        },
      ],
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
    if !output.contains("A tool for glamorous shell scripts") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  strings::first_capture(output, r"gum version v(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::gum::Gum;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Gum {}).run_method(
        &Version::from("0.17.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/charmbracelet/gum/releases/download/v0.17.0/gum_0.17.0_Linux_arm64.tar.gz".into(),
            bin_folder: BinFolder::Subfolder {
              path: "gum_0.17.0_Linux_arm64".into(),
            },
          },
          Method::CompileGoSource {
            import_path: "github.com/charmbracelet/gum@latest".into(),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (Gum {}).run_method(
        &Version::from("0.17.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/charmbracelet/gum/releases/download/v0.17.0/gum_0.17.0_Linux_x86_64.tar.gz".into(),
            bin_folder: BinFolder::Subfolder {
              path: "gum_0.17.0_Linux_x86_64".into(),
            },
          },
          Method::CompileGoSource {
            import_path: "github.com/charmbracelet/gum@latest".into(),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Gum {}).run_method(
        &Version::from("0.17.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/charmbracelet/gum/releases/download/v0.17.0/gum_0.17.0_Darwin_arm64.tar.gz".into(),
            bin_folder: BinFolder::Subfolder {
              path: "gum_0.17.0_Darwin_arm64".into(),
            },
          },
          Method::CompileGoSource {
            import_path: "github.com/charmbracelet/gum@latest".into(),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (Gum {}).run_method(
        &Version::from("0.17.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/charmbracelet/gum/releases/download/v0.17.0/gum_0.17.0_Darwin_x86_64.tar.gz".into(),
            bin_folder: BinFolder::Subfolder {
              path: "gum_0.17.0_Darwin_x86_64".into(),
            },
          },
          Method::CompileGoSource {
            import_path: "github.com/charmbracelet/gum@latest".into(),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (Gum {}).run_method(
        &Version::from("0.17.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/charmbracelet/gum/releases/download/v0.17.0/gum_0.17.0_Windows_arm64.zip".into(),
            bin_folder: BinFolder::Subfolder {
              path: "gum_0.17.0_Windows_arm64".into(),
            },
          },
          Method::CompileGoSource {
            import_path: "github.com/charmbracelet/gum@latest".into(),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Gum {}).run_method(
        &Version::from("0.17.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/charmbracelet/gum/releases/download/v0.17.0/gum_0.17.0_Windows_x86_64.zip".into(),
            bin_folder: BinFolder::Subfolder {
              path: "gum_0.17.0_Windows_x86_64".into(),
            },
          },
          Method::CompileGoSource {
            import_path: "github.com/charmbracelet/gum@latest".into(),
          },
        ],
      };
      assert_eq!(have, want);
    }
  }
}
