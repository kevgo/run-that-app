use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::run::ExecutablePath;
use crate::{run, Log};
use big_s::S;

const ORG: &str = "dominikh";
const REPO: &str = "go-tools";

pub struct StaticCheck {}

impl App for StaticCheck {
  fn name(&self) -> &'static str {
    "staticcheck"
  }

  fn homepage(&self) -> &'static str {
    "https://staticcheck.dev"
  }

  fn run_method(&self, version: &Version, platform: Platform) -> run::Method {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "amd64",
    };
    run::Method::ThisApp {
      install_methods: vec![
        Method::DownloadArchive {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/{version}/staticcheck_{os}_{cpu}.tar.gz"),
          bin_folder: BinFolder::Subfolder { path: S("staticcheck") },
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

  fn analyze_executable(&self, executable: &ExecutablePath, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Usage: staticcheck [flags] [packages]") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn clone(&self) -> Box<dyn App> {
    Box::new(Self {})
  }
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::staticcheck::StaticCheck;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use crate::run;
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
      let want = run::Method::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: S("https://github.com/dominikh/go-tools/releases/download/3.7.0/staticcheck_darwin_arm64.tar.gz"),
            bin_folder: BinFolder::Subfolder { path: S("staticcheck") },
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
      let want = run::Method::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: S("https://github.com/dominikh/go-tools/releases/download/3.7.0/staticcheck_windows_amd64.tar.gz"),
            bin_folder: BinFolder::Subfolder { path: S("staticcheck") },
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
