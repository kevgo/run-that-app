use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::execution::Executable;
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::Platform;
use crate::prelude::*;
use crate::Log;
use const_format::formatcp;

pub struct Exhaustruct {}

const ORG: &str = "GaijinEntertainment";
const REPO: &str = "go-exhaustruct";

impl App for Exhaustruct {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("exhaustruct")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn install_methods(&self, version: &Version, _platform: Platform) -> Vec<installation::Method> {
    vec![Method::CompileGoSource {
      import_path: format!("github.com/{ORG}/{REPO}/v3/cmd/exhaustruct@v{version}"),
    }]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("exhaustruct: Checks if all structure fields are initialized") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

#[cfg(test)]
mod tests {
  use crate::applications::exhaustruct::Exhaustruct;

  #[test]
  fn install_methods() {
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    let have = (Exhaustruct {}).install_methods(
      &Version::from("3.3.0"),
      Platform {
        os: Os::Linux,
        cpu: Cpu::Arm64,
      },
    );
    let want = vec![Method::CompileGoSource {
      import_path: S("github.com/GaijinEntertainment/go-exhaustruct/v3/cmd/exhaustruct@v3.3.0"),
    }];
    assert_eq!(have, want);
  }
}
