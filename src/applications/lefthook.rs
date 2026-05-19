use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, executables, strings};
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct Lefthook {}

const ORG: &str = "evilmartians";
const REPO: &str = "lefthook";

impl AppDefinition for Lefthook {
  fn name(&self) -> ApplicationName {
    "lefthook".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://lefthook.dev")
  }

  fn executable_filename(&self) -> executables::ExecutableNameUnix {
    executables::ExecutableNameUnix::from("lefthook")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "x86_64",
    };
    let os = match platform.os {
      Os::Linux => "Linux",
      Os::MacOS => "MacOS",
      Os::Windows => "Windows",
    };
    let tag = self.tag_format().format_version(version);
    RunMethod::ThisApp {
      install_methods: vec![
        Method::DownloadArchive {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/{tag}/lefthook_{version}_{os}_{cpu}.gz").into(),
          bin_folder: BinFolder::Root,
        },
        Method::CompileGoSource {
          import_path: format!("github.com/{ORG}/{REPO}/v2@{tag}"),
        },
      ],
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
    if !output.contains("Git hooks manager") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn tag_format(&self) -> TagFormat {
    TagFormat::PrefixV
  }
}

fn extract_version(output: &str) -> Result<&str> {
  strings::first_capture(output, r"lefthook version (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod run_method {
    use crate::applications::AppDefinition;
    use crate::applications::lefthook::Lefthook;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (Lefthook {}).run_method(
        &Version::from("2.1.6"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/evilmartians/lefthook/releases/download/v2.1.6/lefthook_2.1.6_Linux_arm64.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileGoSource {
            import_path: S("github.com/evilmartians/lefthook/v2@v2.1.6"),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (Lefthook {}).run_method(
        &Version::from("2.1.6"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/evilmartians/lefthook/releases/download/v2.1.6/lefthook_2.1.6_Linux_x86_64.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileGoSource {
            import_path: S("github.com/evilmartians/lefthook/v2@v2.1.6"),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Lefthook {}).run_method(
        &Version::from("2.1.6"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/evilmartians/lefthook/releases/download/v2.1.6/lefthook_2.1.6_MacOS_arm64.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileGoSource {
            import_path: S("github.com/evilmartians/lefthook/v2@v2.1.6"),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (Lefthook {}).run_method(
        &Version::from("2.1.6"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/evilmartians/lefthook/releases/download/v2.1.6/lefthook_2.1.6_MacOS_x86_64.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileGoSource {
            import_path: S("github.com/evilmartians/lefthook/v2@v2.1.6"),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (Lefthook {}).run_method(
        &Version::from("2.1.6"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/evilmartians/lefthook/releases/download/v2.1.6/lefthook_2.1.6_Windows_arm64.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileGoSource {
            import_path: S("github.com/evilmartians/lefthook/v2@v2.1.6"),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Lefthook {}).run_method(
        &Version::from("2.1.6"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/evilmartians/lefthook/releases/download/v2.1.6/lefthook_2.1.6_Windows_x86_64.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileGoSource {
            import_path: S("github.com/evilmartians/lefthook/v2@v2.1.6"),
          },
        ],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("lefthook version 2.1.6"), Ok("2.1.6"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
