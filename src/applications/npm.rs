use super::nodejs::NodeJS;
use super::{AnalyzeResult, AppDefinition};
use crate::configuration::{ApplicationName, Version};
use crate::platform::Platform;
use crate::prelude::*;
use crate::run::{ExecutableArgs, ExecutableNameUnix, ExecutablePath};
use crate::{run, Log};

pub struct Npm {}

impl AppDefinition for Npm {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("npm")
  }

  fn homepage(&self) -> &'static str {
    "https://www.npmjs.com"
  }

  fn run_method(&self, _version: &Version, _platform: Platform) -> run::Method {
    run::Method::OtherAppDefaultExecutable {
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

  fn analyze_executable(&self, executable: &ExecutablePath, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output_args(&["help", "npm"], log)?;
    if !output.contains("javascript package manager") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // Npm is versioned together with NodeJS. The actual version of npm is therefore not relevant here.
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn clone(&self) -> Box<dyn AppDefinition> {
    Box::new(Self {})
  }
}

fn app_to_install() -> NodeJS {
  NodeJS {}
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::nodejs::NodeJS;
    use crate::applications::npm::Npm;
    use crate::applications::AppDefinition;
    use crate::configuration::Version;
    use crate::platform::{Cpu, Os, Platform};
    use crate::run;
    use crate::run::ExecutableArgs;
    use crate::run::ExecutableNameUnix;

    #[test]
    #[cfg(unix)]
    fn linux_arm() {
      let have = (Npm {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = run::Method::OtherAppDefaultExecutable {
        app_definition: Box::new(NodeJS {}),
        args: ExecutableArgs::OneOfTheseInAppFolder {
          options: vec!["node_modules/npm/bin/npm-cli.js", "lib/node_modules/npm/bin/npm-cli.js"],
        },
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
      let want = run::Method::OtherAppDefaultExecutable {
        app: Box::new(NodeJS {}),
        args: OtherAppArgs::OneOfTheseInAppFolder {
          options: vec!["node_modules/npm/bin/npm-cli.js", "lib/node_modules/npm/bin/npm-cli.js"],
        },
      };
      assert_eq!(have, want);
    }
  }
}
