use super::nodejs::NodeJS;
use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::platform::Platform;
use crate::prelude::*;
use crate::run::{ExecutableNameUnix, ExecutablePath};
use crate::{run, Log};

pub struct Npx {}

impl App for Npx {
  fn name(&self) -> &'static str {
    "npx"
  }

  fn homepage(&self) -> &'static str {
    "https://www.npmjs.com"
  }

  fn run_method(&self, _version: &Version, _platform: Platform) -> run::Method {
    run::Method::OtherAppOtherExecutable {
      app: Box::new(app_to_install()),
      executable_name: ExecutableNameUnix::from("npx"),
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    app_to_install().latest_installable_version(log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    app_to_install().installable_versions(amount, log)
  }

  fn analyze_executable(&self, executable: &ExecutablePath, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Run a command from a local or remote npm package") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // Npx is versioned together with NodeJS. The actual version of npm is therefore not relevant here.
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn clone(&self) -> Box<dyn App> {
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
    use crate::applications::npx::Npx;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::platform::{Cpu, Os, Platform};
    use crate::run::{self, ExecutableNameUnix};

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
      let want = run::Method::OtherAppOtherExecutable {
        app: Box::new(NodeJS {}),
        executable_name: ExecutableNameUnix::from("npx"),
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
      let want = run::Method::OtherAppOtherExecutable {
        app: Box::new(NodeJS {}),
        executable_name: ExecutableNameUnix::from("npx"),
      };
      assert_eq!(have, want);
    }
  }
}
