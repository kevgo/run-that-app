use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::Platform;
use crate::{Log, strings};
use const_format::formatcp;

#[derive(Clone)]
pub struct KeepSorted {}

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
    let tag = self.tag_format().format_version(version);
    RunMethod::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: format!("github.com/{ORG}/{REPO}@{tag}"),
      }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, &self.tag_format(), log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, &self.tag_format(), log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("The options keep-sorted will use to sort.") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match strings::capture_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn tag_format(&self) -> TagFormat {
    TagFormat::PrefixV
  }
}

#[cfg(test)]
mod tests {

  mod run_method {
    use crate::applications::AppDefinition;
    use crate::applications::keep_sorted::KeepSorted;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (KeepSorted {}).run_method(
        &Version::from("0.7.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/google/keep-sorted@v0.7.1"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (KeepSorted {}).run_method(
        &Version::from("0.7.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/google/keep-sorted@v0.7.1"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (KeepSorted {}).run_method(
        &Version::from("0.7.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/google/keep-sorted@v0.7.1"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (KeepSorted {}).run_method(
        &Version::from("0.7.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/google/keep-sorted@v0.7.1"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (KeepSorted {}).run_method(
        &Version::from("0.7.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/google/keep-sorted@v0.7.1"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (KeepSorted {}).run_method(
        &Version::from("0.7.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/google/keep-sorted@v0.7.1"),
        }],
      };
      assert_eq!(have, want);
    }
  }
}
