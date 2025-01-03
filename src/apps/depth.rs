use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::Log;
use const_format::formatcp;

pub struct Depth {}

const ORG: &str = "KyleBanks";
const REPO: &str = "depth";

impl App for Depth {
  fn name(&self) -> AppName {
    AppName::from("depth")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn install_methods(&self) -> Vec<install::Method> {
    vec![Method::DownloadExecutable(self), Method::CompileGoSource(self)]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("resolves dependencies of internal (stdlib) packages.") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // as of 1.2.1 depth doesn't display the version of the installed executable
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

impl install::CompileGoSource for Depth {
  fn import_path(&self, version: &Version) -> String {
    format!("github.com/{ORG}/{REPO}/cmd/depth@v{version}")
  }
}

impl install::DownloadExecutable for Depth {
  fn download_url(&self, version: &Version, platform: Platform) -> String {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "aarch64", // the "arm" binaries don't run on Apple Silicon
      Cpu::Intel64 => "amd64",
    };
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "windows",
    };
    format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/depth_{version}_{os}_{cpu}",)
  }
}

#[cfg(test)]
mod tests {
  use crate::config::Version;
  use crate::install::DownloadExecutable;
  use crate::platform::{Cpu, Os, Platform};

  #[test]
  fn artifact_url() {
    let depth = super::Depth {};
    let platform = Platform {
      os: Os::Linux,
      cpu: Cpu::Intel64,
    };
    let have = depth.download_url(&Version::from("1.2.1"), platform);
    let want = "https://github.com/KyleBanks/depth/releases/download/v1.2.1/depth_1.2.1_linux_amd64";
    assert_eq!(have, want);
  }
}
