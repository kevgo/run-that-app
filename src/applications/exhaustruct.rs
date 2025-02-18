use super::{AnalyzeResult, AppDefinition};
use crate::configuration::Version;
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::Platform;
use crate::prelude::*;
use crate::executable::Executable;
use crate::{executable, Log};
use const_format::formatcp;

pub(crate) struct Exhaustruct {}

const ORG: &str = "GaijinEntertainment";
const REPO: &str = "go-exhaustruct";

impl AppDefinition for Exhaustruct {
  fn name(&self) -> &'static str {
    "exhaustruct"
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn run_method(&self, version: &Version, _platform: Platform) -> executable::Method {
    executable::Method::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: format!("github.com/{ORG}/{REPO}/v3/cmd/exhaustruct@v{version}"),
      }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("exhaustruct: Checks if all structure fields are initialized") {
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
  use crate::applications::exhaustruct::Exhaustruct;
  use crate::executable;

  #[test]
  fn install_methods() {
    use crate::applications::AppDefinition;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    let have = (Exhaustruct {}).run_method(
      &Version::from("3.3.0"),
      Platform {
        os: Os::Linux,
        cpu: Cpu::Arm64,
      },
    );
    let want = executable::Method::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: S("github.com/GaijinEntertainment/go-exhaustruct/v3/cmd/exhaustruct@v3.3.0"),
      }],
    };
    assert_eq!(have, want);
  }
}
