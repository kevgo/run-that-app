use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
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
    vec![
      Method::DownloadArchive {
        url: archive_url(version, platform),
        executable_path: self.executable_filename(platform),
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

fn archive_url(version: &Version, platform: Platform) -> String {
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

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"scc version (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod archive_url {
    use crate::configuration::Version;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
      let have = super::super::archive_url(&Version::from("3.2.0"), platform);
      let want = "https://github.com/boyter/scc/releases/download/v3.2.0/scc_Darwin_arm64.tar.gz";
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let platform = Platform { os: Os::Linux, cpu: Cpu::Intel64 };
      let have = super::super::archive_url(&Version::from("3.2.0"), platform);
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
