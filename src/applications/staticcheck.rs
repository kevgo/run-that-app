use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::Log;
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};

const ORG: &str = "dominikh";
const REPO: &str = "go-tools";

#[derive(Clone)]
pub(crate) struct StaticCheck {}

impl AppDefinition for StaticCheck {
  fn name(&self) -> ApplicationName {
    "staticcheck".into()
  }

  fn homepage(&self) -> &'static str {
    "https://staticcheck.dev"
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "amd64",
    };
    RunMethod::ThisApp {
      install_methods: vec![
        Method::DownloadArchive {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/{version}/staticcheck_{os}_{cpu}.tar.gz").into(),
          bin_folder: BinFolder::Subfolder { path: "staticcheck".into() },
        },
        Method::CompileGoSource {
          import_path: format!("honnef.co/go/tools/cmd/staticcheck@{version}"),
        },
      ],
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("Usage: staticcheck [flags] [packages]") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::staticcheck::StaticCheck;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (StaticCheck {}).run_method(
        &Version::from("3.7.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/dominikh/go-tools/releases/download/3.7.0/staticcheck_darwin_arm64.tar.gz".into(),
            bin_folder: BinFolder::Subfolder { path: "staticcheck".into() },
          },
          Method::CompileGoSource {
            import_path: S("honnef.co/go/tools/cmd/staticcheck@3.7.0"),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (StaticCheck {}).run_method(
        &Version::from("3.7.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/dominikh/go-tools/releases/download/3.7.0/staticcheck_windows_amd64.tar.gz".into(),
            bin_folder: BinFolder::Subfolder { path: "staticcheck".into() },
          },
          Method::CompileGoSource {
            import_path: S("honnef.co/go/tools/cmd/staticcheck@3.7.0"),
          },
        ],
      };
      assert_eq!(have, want);
    }
  }
}
