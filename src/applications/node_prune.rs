use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_tags;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::Log;
use const_format::formatcp;

pub struct NodePrune {}

const ORG: &str = "tj";
const REPO: &str = "node-prune";

impl App for NodePrune {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("node-prune")
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

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    vec![
      Method::DownloadExecutable {
        url: download_url(version, platform),
      },
      Method::CompileGoSource {
        import_path: format!("github.com/tj/node-prune@v{version}"),
      },
    ]
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

fn download_url(version: &Version, platform: Platform) -> String {
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

#[cfg(test)]
mod tests {

  mod artifact_url {
    use crate::configuration::Version;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn windows_intel64() {
      let platform = Platform { os: Os::Linux, cpu: Cpu::Intel64 };
      let have = super::super::download_url(&Version::from("1.0.1"), platform);
      let want = "https://github.com/tj/node-prune/releases/download/v1.0.1/node-prune_1.0.1_linux_amd64.tar.gz";
      assert_eq!(have, want);
    }
  }
}
