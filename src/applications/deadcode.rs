use super::{AnalyzeResult, AppDefinition};
use crate::Log;
use crate::applications::ApplicationName;
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::pkg_go_dev;
use crate::installation::Method;
use crate::platform::Platform;

#[derive(Clone)]
pub struct Deadcode {}

const PKG_NAME: &str = "golang.org/x/tools";

impl AppDefinition for Deadcode {
  fn name(&self) -> ApplicationName {
    "deadcode".into()
  }

  fn homepage(&self) -> &'static str {
    "https://pkg.go.dev/golang.org/x/tools/cmd/deadcode"
  }

  fn run_method(&self, version: &Version, _platform: Platform) -> RunMethod {
    let tag = self.tag_format().format_version(version);
    RunMethod::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: format!("golang.org/x/tools/cmd/deadcode@{tag}"),
      }],
    }
  }

  fn latest_installable_version(&self, _log: Log) -> Result<Version> {
    pkg_go_dev::latest(PKG_NAME, &self.tag_format())
  }

  fn installable_versions(&self, amount: usize, _log: Log) -> Result<Vec<Version>> {
    pkg_go_dev::versions(PKG_NAME, amount, &self.tag_format())
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("The deadcode command reports unreachable functions in Go programs") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // as of 0.16.1 deadcode does not display the version of the installed executable
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
    use crate::applications::deadcode::Deadcode;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (Deadcode {}).run_method(
        &Version::from("0.16.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("golang.org/x/tools/cmd/deadcode@v0.16.1"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_intel() {
      let have = (Deadcode {}).run_method(
        &Version::from("0.16.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("golang.org/x/tools/cmd/deadcode@v0.16.1"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Deadcode {}).run_method(
        &Version::from("0.16.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("golang.org/x/tools/cmd/deadcode@v0.16.1"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_intel() {
      let have = (Deadcode {}).run_method(
        &Version::from("0.16.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("golang.org/x/tools/cmd/deadcode@v0.16.1"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (Deadcode {}).run_method(
        &Version::from("0.16.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("golang.org/x/tools/cmd/deadcode@v0.16.1"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Deadcode {}).run_method(
        &Version::from("0.16.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::CompileGoSource {
          import_path: S("golang.org/x/tools/cmd/deadcode@v0.16.1"),
        }],
      };
      assert_eq!(have, want);
    }
  }
}
