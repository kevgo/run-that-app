use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::Platform;
use crate::run::Executable;
use crate::Log;
use crate::{prelude::*, run};
use const_format::formatcp;

pub struct Goda {}

const ORG: &str = "loov";
const REPO: &str = "goda";

impl App for Goda {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("goda")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, _platform: Platform) -> run::Method {
    run::Method::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: format!("github.com/{ORG}/{REPO}@v{version}"),
      }],
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("help", log)?;
    if !output.contains("Print dependency graph") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // as of 0.5.7 goda has no way to determine the version of the installed executable
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

#[cfg(test)]
mod tests {
  use crate::run;

  #[test]
  fn install_methods() {
    use crate::applications::goda::Goda;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    let have = (Goda {}).run_method(
      &Version::from("0.5.9"),
      Platform {
        os: Os::MacOS,
        cpu: Cpu::Intel64,
      },
    );
    let want = run::Method::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: S("github.com/loov/goda@v0.5.9"),
      }],
    };
    assert_eq!(have, want);
  }
}
