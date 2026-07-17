use super::nodejs::NodeJS;
use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::Log;
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, ExecutableArgs, RunMethod};
use crate::platform::Platform;
use const_format::formatcp;
use std::path::MAIN_SEPARATOR;

#[derive(Clone)]
pub struct Npm {}

impl AppDefinition for Npm {
  fn name(&self) -> ApplicationName {
    "npm".into()
  }

  fn homepage(&self) -> &'static str {
    "https://www.npmjs.com"
  }

  fn run_method(&self, _version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::OtherAppDefaultExecutable {
      app_definition: Box::new(NodeJS {}),
      args: ExecutableArgs::OneOfTheseInAppFolder {
        options: vec![
          // on Windows, npm is a batch file
          formatcp!("bin{MAIN_SEPARATOR}npm.cmd"),
          // on Unix, npm is a script
          formatcp!("bin{MAIN_SEPARATOR}npm"),
        ],
      },
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    app_to_install().latest_installable_version(log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    app_to_install().installable_versions(amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["help", "npm"], log)?;
    if !output.contains("javascript package manager") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // Npm is versioned together with NodeJS. The actual version of npm is therefore not relevant here.
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
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
    use crate::executables::{ExecutableArgs, RunMethod};
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
      let want = RunMethod::OtherAppDefaultExecutable {
        app_definition: Box::new(NodeJS {}),
        args: ExecutableArgs::OneOfTheseInAppFolder { options: vec!["bin/npm"] },
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
      let want = RunMethod::OtherAppDefaultExecutable {
        app_definition: Box::new(NodeJS {}),
        args: ExecutableArgs::OneOfTheseInAppFolder { options: vec!["bin/npm"] },
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
      let want = RunMethod::OtherAppDefaultExecutable {
        app_definition: Box::new(NodeJS {}),
        args: ExecutableArgs::OneOfTheseInAppFolder { options: vec!["bin/npm"] },
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
      let want = RunMethod::OtherAppDefaultExecutable {
        app_definition: Box::new(NodeJS {}),
        args: ExecutableArgs::OneOfTheseInAppFolder { options: vec!["bin/npm"] },
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
      let want = RunMethod::OtherAppDefaultExecutable {
        app_definition: Box::new(NodeJS {}),
        args: ExecutableArgs::OneOfTheseInAppFolder { options: vec!["bin\\npm"] },
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
      let want = RunMethod::OtherAppDefaultExecutable {
        app_definition: Box::new(NodeJS {}),
        args: ExecutableArgs::OneOfTheseInAppFolder { options: vec!["bin\\npm"] },
      };
      assert_eq!(have, want);
    }
  }
}
