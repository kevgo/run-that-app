use super::{AnalyzeResult, AppDefinition};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_tags;
use crate::installation::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::run::ExecutablePath;
use crate::{run, Log};
use const_format::formatcp;

pub(crate) struct NodePrune {}

const ORG: &str = "tj";
const REPO: &str = "node-prune";

impl AppDefinition for NodePrune {
  fn name(&self) -> &'static str {
    "node-prune"
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
        Method::DownloadExecutable {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/node-prune_{version}_{os}_{cpu}.tar.gz"),
        },
        Method::CompileGoSource {
          import_path: format!("github.com/tj/node-prune@v{version}"),
        },
      ],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    let tags = github_tags::all(ORG, REPO, amount, log)?;
    Ok(tags.into_iter().map(Version::from).collect())
  }

  fn analyze_executable(&self, executable: &ExecutablePath, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Glob of files that should not be pruned") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn clone(&self) -> Box<dyn AppDefinition> {
    Box::new(Self {})
  }
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::node_prune::NodePrune;
    use crate::applications::AppDefinition;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use crate::run;
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (NodePrune {}).run_method(
        &Version::from("1.0.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![
          Method::DownloadExecutable {
            url: S("https://github.com/tj/node-prune/releases/download/v1.0.1/node-prune_1.0.1_linux_amd64.tar.gz"),
          },
          Method::CompileGoSource {
            import_path: S("github.com/tj/node-prune@v1.0.1"),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (NodePrune {}).run_method(
        &Version::from("1.0.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![
          Method::DownloadExecutable {
            url: S("https://github.com/tj/node-prune/releases/download/v1.0.1/node-prune_1.0.1_windows_amd64.tar.gz"),
          },
          Method::CompileGoSource {
            import_path: S("github.com/tj/node-prune@v1.0.1"),
          },
        ],
      };
      assert_eq!(have, want);
    }
  }
}
