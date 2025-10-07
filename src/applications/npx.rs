use super::nodejs::NodeJS;
use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::Log;
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, ExecutableArgs, RunMethod};
use crate::platform::Platform;

#[derive(Clone)]
pub(crate) struct Npx {}

impl AppDefinition for Npx {
  fn name(&self) -> ApplicationName {
    "npx".into()
  }

  fn homepage(&self) -> &'static str {
    "https://www.npmjs.com"
  }

  fn run_method(&self, _version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::OtherAppDefaultExecutable {
      app_definition: Box::new(app_to_install()),
      args: ExecutableArgs::OneOfTheseInAppFolder {
        options: vec!["node_modules/npm/bin/npx-cli.js", "lib/node_modules/npm/bin/npx-cli.js"],
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
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("Run a command from a local or remote npm package") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // Npx is versioned together with NodeJS. The actual version of npm is therefore not relevant here.
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

fn app_to_install() -> NodeJS {
  NodeJS {}
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::nodejs::NodeJS;
    use crate::applications::npx::Npx;
    use crate::configuration::Version;
    use crate::executables::{ExecutableArgs, RunMethod};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    #[cfg(unix)]
    fn linux_arm() {
      let have = (Npx {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::OtherAppDefaultExecutable {
        app_definition: Box::new(NodeJS {}),
        args: ExecutableArgs::OneOfTheseInAppFolder {
          options: vec!["node_modules/npm/bin/npx-cli.js", "lib/node_modules/npm/bin/npx-cli.js"],
        },
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_intel() {
      let have = (Npx {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::OtherAppDefaultExecutable {
        app_definition: Box::new(NodeJS {}),
        args: ExecutableArgs::OneOfTheseInAppFolder {
          options: vec!["node_modules/npm/bin/npx-cli.js", "lib/node_modules/npm/bin/npx-cli.js"],
        },
      };
      assert_eq!(have, want);
    }
  }
}
