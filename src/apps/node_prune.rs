use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_tags;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::Log;
use const_format::formatcp;

pub struct NodePrune {}

const ORG: &str = "tj";
const REPO: &str = "node-prune";

impl App for NodePrune {
  fn name(&self) -> AppName {
    AppName::from("node-prune")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    let tags = github_tags::all(ORG, REPO, 1, log)?;
    let Some(tag) = tags.into_iter().nth(0) else {
      return Err(UserError::NoVersionsFound { app: self.name().to_string() });
    };
    Ok(Version::from(tag))
  }

  fn install_methods(&self) -> Vec<install::Method> {
    vec![Method::DownloadExecutable(self), Method::CompileGoSource(self)]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    let tags = github_tags::all(ORG, REPO, amount, log)?;
    Ok(tags.into_iter().map(Version::from).collect())
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Glob of files that should not be pruned") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

impl install::DownloadExecutable for NodePrune {
  fn download_url(&self, version: &Version, platform: Platform) -> String {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "amd64",
    };
    format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/node-prune_{version}_{os}_{cpu}.tar.gz")
  }
}

impl install::CompileGoSource for NodePrune {
  fn import_path(&self, version: &Version) -> String {
    format!("github.com/tj/node-prune@v{version}")
  }
}

#[cfg(test)]
mod tests {

  mod artifact_url {
    use crate::config::Version;
    use crate::install::DownloadExecutable;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn windows_intel64() {
      let node_prune = super::super::NodePrune {};
      let platform = Platform {
        os: Os::Linux,
        cpu: Cpu::Intel64,
      };
      let have = node_prune.download_url(&Version::from("1.0.1"), platform);
      let want = "https://github.com/tj/node-prune/releases/download/v1.0.1/node-prune_1.0.1_linux_amd64.tar.gz";
      assert_eq!(have, want);
    }
  }
}
