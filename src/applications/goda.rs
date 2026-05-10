use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::Log;
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::Platform;
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct Goda {}

const ORG: &str = "loov";
const REPO: &str = "goda";

impl AppDefinition for Goda {
  fn name(&self) -> ApplicationName {
    "goda".into()
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

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, &self.tag_format(), log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, &self.tag_format(), log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["help"], log)?;
    if !output.contains("Print dependency graph") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // as of 0.5.7 goda has no way to determine the version of the installed executable
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn tag_format(&self) -> TagFormat {
    TagFormat::PrefixV
  }
}

#[cfg(test)]
mod tests {
  mod run_method {
    use crate::applications::AppDefinition;
    use crate::applications::goda::Goda;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (Goda {}).run_method(
        &Version::from("0.5.9"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/loov/goda@v0.5.9"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (Goda {}).run_method(
        &Version::from("0.5.9"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/loov/goda@v0.5.9"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Goda {}).run_method(
        &Version::from("0.5.9"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/loov/goda@v0.5.9"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (Goda {}).run_method(
        &Version::from("0.5.9"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/loov/goda@v0.5.9"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (Goda {}).run_method(
        &Version::from("0.5.9"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/loov/goda@v0.5.9"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Goda {}).run_method(
        &Version::from("0.5.9"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("github.com/loov/goda@v0.5.9"),
        }],
      };
      assert_eq!(have, want);
    }
  }
}
