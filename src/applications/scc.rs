use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::execution::Executable;
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::{regexp, Log};
use const_format::formatcp;

pub struct Scc {}

const ORG: &str = "boyter";
const REPO: &str = "scc";

impl App for Scc {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("scc")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    let os = match platform.os {
      Os::Linux => "Linux",
      Os::MacOS => "Darwin",
      Os::Windows => "Windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "x86_64",
    };
    vec![
      Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/scc_{os}_{cpu}.tar.gz"),
        path_in_archive: self.executable_filename(platform),
      },
      Method::CompileGoSource {
        import_path: format!("github.com/{ORG}/{REPO}/v3@v{version}"),
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
    if !output.contains("Count lines of code in a directory with complexity estimation") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("--version", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"scc version (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::scc::Scc;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (Scc {}).install_methods(
        &Version::from("3.2.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = vec![
        Method::DownloadArchive {
          url: S("https://github.com/boyter/scc/releases/download/v3.2.0/scc_Darwin_arm64.tar.gz"),
          path_in_archive: S("scc"),
        },
        Method::CompileGoSource {
          import_path: S("github.com/boyter/scc/v3@v3.2.0"),
        },
      ];
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Scc {}).install_methods(
        &Version::from("3.2.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![
        Method::DownloadArchive {
          url: S("https://github.com/boyter/scc/releases/download/v3.2.0/scc_Windows_x86_64.tar.gz"),
          path_in_archive: S("scc.exe"),
        },
        Method::CompileGoSource {
          import_path: S("github.com/boyter/scc/v3@v3.2.0"),
        },
      ];
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("scc version 3.2.0"), Ok("3.2.0"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
