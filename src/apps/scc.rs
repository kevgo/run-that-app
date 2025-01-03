use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};
use const_format::formatcp;

pub struct Scc {}

const ORG: &str = "boyter";
const REPO: &str = "scc";

impl App for Scc {
  fn name(&self) -> AppName {
    AppName::from("scc")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn install_methods(&self) -> Vec<install::Method> {
    vec![Method::DownloadArchive(self), Method::CompileGoSource(self)]
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

impl install::DownloadArchive for Scc {
  fn archive_url(&self, version: &Version, platform: Platform) -> String {
    let os = match platform.os {
      Os::Linux => "Linux",
      Os::MacOS => "Darwin",
      Os::Windows => "Windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "x86_64",
    };
    format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/scc_{os}_{cpu}.tar.gz")
  }

  fn executable_path_in_archive(&self, _version: &Version, platform: Platform) -> String {
    self.executable_filename(platform)
  }
}

impl install::CompileGoSource for Scc {
  fn import_path(&self, version: &Version) -> String {
    format!("github.com/{ORG}/{REPO}/v3@v{version}")
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"scc version (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::apps::UserError;

  mod archive_url {
    use crate::config::Version;
    use crate::install::DownloadArchive;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let scc = super::super::Scc {};
      let platform = Platform {
        os: Os::MacOS,
        cpu: Cpu::Arm64,
      };
      let have = scc.archive_url(&Version::from("3.2.0"), platform);
      let want = "https://github.com/boyter/scc/releases/download/v3.2.0/scc_Darwin_arm64.tar.gz";
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let scc = super::super::Scc {};
      let platform = Platform {
        os: Os::Linux,
        cpu: Cpu::Intel64,
      };
      let have = scc.archive_url(&Version::from("3.2.0"), platform);
      let want = "https://github.com/boyter/scc/releases/download/v3.2.0/scc_Linux_x86_64.tar.gz";
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("scc version 3.2.0"), Ok("3.2.0"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
