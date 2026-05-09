use super::nodejs::NodeJS;
use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::Log;
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, ExecutableArgs, RunMethod};
use crate::platform::Platform;

#[derive(Clone)]
pub(crate) struct Npm {}

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
        options: vec!["node_modules/npm/bin/npm-cli.js", "lib/node_modules/npm/bin/npm-cli.js"],
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

    fn want() -> RunMethod {
      RunMethod::OtherAppDefaultExecutable {
        app_definition: Box::new(NodeJS {}),
        args: ExecutableArgs::OneOfTheseInAppFolder {
          options: vec!["node_modules/npm/bin/npm-cli.js", "lib/node_modules/npm/bin/npm-cli.js"],
        },
      }
    }

    #[test]
    fn linux_arm() {
      assert_eq!(
        (Npm {}).run_method(
          &Version::from("20.10.0"),
          Platform {
            os: Os::Linux,
            cpu: Cpu::Arm64,
          },
        ),
        want(),
      );
    }

    #[test]
    fn linux_intel() {
      assert_eq!(
        (Npm {}).run_method(
          &Version::from("20.10.0"),
          Platform {
            os: Os::Linux,
            cpu: Cpu::Intel64,
          },
        ),
        want(),
      );
    }

    #[test]
    fn macos_arm() {
      assert_eq!(
        (Npm {}).run_method(
          &Version::from("20.10.0"),
          Platform {
            os: Os::MacOS,
            cpu: Cpu::Arm64,
          },
        ),
        want(),
      );
    }

    #[test]
    fn macos_intel() {
      assert_eq!(
        (Npm {}).run_method(
          &Version::from("20.10.0"),
          Platform {
            os: Os::MacOS,
            cpu: Cpu::Intel64,
          },
        ),
        want(),
      );
    }

    #[test]
    fn windows_arm() {
      assert_eq!(
        (Npm {}).run_method(
          &Version::from("20.10.0"),
          Platform {
            os: Os::Windows,
            cpu: Cpu::Arm64,
          },
        ),
        want(),
      );
    }

    #[test]
    fn windows_intel() {
      assert_eq!(
        (Npm {}).run_method(
          &Version::from("20.10.0"),
          Platform {
            os: Os::Windows,
            cpu: Cpu::Intel64,
          },
        ),
        want(),
      );
    }
  }
}
