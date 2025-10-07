use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::Platform;
use crate::{Log, regexp};
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct KeepSorted {}

const ORG: &str = "google";
const REPO: &str = "keep-sorted";

impl AppDefinition for KeepSorted {
  fn name(&self) -> ApplicationName {
    "keep-sorted".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: format!("github.com/{ORG}/{REPO}@v{version}"),
      }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("The options keep-sorted will use to sort.") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"v(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;
  use crate::executables::RunMethod;

  #[test]
  fn install_methods() {
    use crate::applications::AppDefinition;
    use crate::applications::alphavet::Alphavet;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    let have = (Alphavet {}).run_method(
      &Version::from("0.1.0"),
      Platform {
        os: Os::Linux,
        cpu: Cpu::Arm64,
      },
    );
    let want = RunMethod::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: S("github.com/skx/alphavet/cmd/alphavet@v0.1.0"),
      }],
    };
    assert_eq!(have, want);
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("v0.7.1"), Ok("0.7.1"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
