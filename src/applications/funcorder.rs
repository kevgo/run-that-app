use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::Log;
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::Platform;
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct FuncOrder {}

const ORG: &str = "manuelarte";
const REPO: &str = "funcorder";
const TAG_PREFIX: &str = "v";

impl AppDefinition for FuncOrder {
  fn name(&self) -> ApplicationName {
    REPO.into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::ThisApp {
      install_methods: vec![Method::GoCompileSource {
        import_path: format!("github.com/{ORG}/{REPO}@{TAG_PREFIX}{version}"),
      }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, TAG_PREFIX, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, TAG_PREFIX, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("checks the order of functions, methods, and constructors") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

#[cfg(test)]
mod tests {
  mod run_method {
    use crate::applications::AppDefinition;
    use crate::applications::funcorder::FuncOrder;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (FuncOrder {}).run_method(
        &Version::from("0.8.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::GoCompileSource {
          import_path: S("github.com/manuelarte/funcorder@v0.8.0"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (FuncOrder {}).run_method(
        &Version::from("0.8.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::GoCompileSource {
          import_path: S("github.com/manuelarte/funcorder@v0.8.0"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (FuncOrder {}).run_method(
        &Version::from("0.8.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::GoCompileSource {
          import_path: S("github.com/manuelarte/funcorder@v0.8.0"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (FuncOrder {}).run_method(
        &Version::from("0.8.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::GoCompileSource {
          import_path: S("github.com/manuelarte/funcorder@v0.8.0"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (FuncOrder {}).run_method(
        &Version::from("0.8.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::GoCompileSource {
          import_path: S("github.com/manuelarte/funcorder@v0.8.0"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (FuncOrder {}).run_method(
        &Version::from("0.8.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::GoCompileSource {
          import_path: S("github.com/manuelarte/funcorder@v0.8.0"),
        }],
      };
      assert_eq!(have, want);
    }
  }
}
