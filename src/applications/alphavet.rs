use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::execution::Executable;
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::Platform;
use crate::prelude::*;
use crate::Log;
use const_format::formatcp;

pub struct Alphavet {}

const ORG: &str = "skx";
const REPO: &str = "alphavet";

impl App for Alphavet {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("alphavet")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn install_methods(&self, version: &Version, _platform: Platform) -> Vec<installation::Method> {
    vec![Method::CompileGoSource {
      import_path: format!("github.com/{ORG}/{REPO}/cmd/alphavet@v{version}"),
    }]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Checks that functions are ordered alphabetically within packages") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // as of 0.1.0 the -V switch of alphavet is broken
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn install_methods() {
    use crate::applications::alphavet::Alphavet;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    let have = (Alphavet {}).install_methods(
      &Version::from("0.1.0"),
      Platform {
        os: Os::Linux,
        cpu: Cpu::Arm64,
      },
    );
    let want = vec![Method::CompileGoSource {
      import_path: S("github.com/skx/alphavet/cmd/alphavet@v0.1.0"),
    }];
    assert_eq!(have, want);
  }
}
