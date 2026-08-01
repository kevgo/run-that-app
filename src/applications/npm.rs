use super::nodejs::NodeJS;
use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::platform::{Os, Platform};
use crate::{Log, strings, subshell};

#[derive(Clone)]
pub struct Npm {}

impl AppDefinition for Npm {
  fn name(&self) -> ApplicationName {
    "npm".into()
  }

  fn homepage(&self) -> &'static str {
    "https://www.npmjs.com"
  }

  fn run_method(&self, _version: &Version, platform: Platform) -> RunMethod {
    RunMethod::OtherAppShellScript {
      carrier: Box::new(NodeJS {}),
      script_name: match platform.os {
        Os::Linux | Os::MacOS => "npm",
        Os::Windows => "npm.cmd",
      },
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    app_to_install().latest_installable_version(log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    app_to_install().installable_versions(amount, log)
  }

  fn analyze_executable(&self, executable: &Executable) -> Result<AnalyzeResult> {
    let output = subshell::capture_output(executable, &["help", "npm"])?;
    if !output.contains("javascript package manager") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // Npm is versioned together with NodeJS. The actual version of npm is therefore not relevant here.
    match strings::first_version(&subshell::capture_output(executable, &["--version"])?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn tag_format(&self) -> TagFormat {
    app_to_install().tag_format()
  }
}

fn app_to_install() -> NodeJS {
  NodeJS {}
}

#[cfg(test)]
mod tests {

  mod run_method {
    use crate::applications::AppDefinition;
    use crate::applications::nodejs::NodeJS;
    use crate::applications::npm::Npm;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    #[cfg(not(windows))]
    fn linux_arm() {
      let have = (Npm {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npm",
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn linux_intel() {
      let have = (Npm {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npm",
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn macos_arm() {
      let have = (Npm {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npm",
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn macos_intel() {
      let have = (Npm {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npm",
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_arm() {
      let have = (Npm {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npm.cmd",
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_intel() {
      let have = (Npm {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npm.cmd",
      };
      assert_eq!(have, want);
    }
  }
}
