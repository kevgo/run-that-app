use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::execution::Executable;
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::{installation, Log};

const ORG: &str = "dominikh";
const REPO: &str = "go-tools";

pub struct StaticCheck {}

impl App for StaticCheck {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("staticcheck")
  }

  fn homepage(&self) -> &'static str {
    "https://staticcheck.dev"
  }

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "amd64",
    };
    vec![
      Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/{version}/staticcheck_{os}_{cpu}.tar.gz"),
        path_in_archive: format!("staticcheck/{}", self.executable_filename(platform)),
      },
      Method::CompileGoSource {
        import_path: format!("honnef.co/go/tools/cmd/staticcheck@{version}"),
      },
    ]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Usage: staticcheck [flags] [packages]") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::staticcheck::StaticCheck;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (StaticCheck {}).install_methods(
        &Version::from("3.7.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = vec![
        Method::DownloadArchive {
          url: S("https://github.com/dominikh/go-tools/releases/download/3.7.0/staticcheck_darwin_arm64.tar.gz"),
          path_in_archive: S("staticcheck/staticcheck"),
        },
        Method::CompileGoSource {
          import_path: S("honnef.co/go/tools/cmd/staticcheck@3.7.0"),
        },
      ];
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (StaticCheck {}).install_methods(
        &Version::from("3.7.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![
        Method::DownloadArchive {
          url: S("https://github.com/dominikh/go-tools/releases/download/3.7.0/staticcheck_windows_amd64.tar.gz"),
          path_in_archive: S("staticcheck/staticcheck.exe"),
        },
        Method::CompileGoSource {
          import_path: S("honnef.co/go/tools/cmd/staticcheck@3.7.0"),
        },
      ];
      assert_eq!(have, want);
    }
  }
}
