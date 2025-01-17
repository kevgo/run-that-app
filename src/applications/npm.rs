use super::nodejs::NodeJS;
use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::installation::{self, Method};
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{applications, Log};
use std::path;

pub struct Npm {}

impl App for Npm {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("npm")
  }

  fn homepage(&self) -> &'static str {
    "https://www.npmjs.com"
  }

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    vec![Method::ExecutableInAnotherApp {
      other_app: Box::new(app_to_install()),
      executable_path: format!(
        "node-v{version}-{os}-{cpu}{sep}bin{sep}{executable}",
        os = applications::nodejs::os_text(platform.os),
        cpu = applications::nodejs::cpu_text(platform.cpu),
        sep = path::MAIN_SEPARATOR,
        executable = self.executable_filename(platform)
      ),
    }]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    app_to_install().latest_installable_version(log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    app_to_install().installable_versions(amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output_args(&["help", "npm"], log)?;
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

  mod install_methods {
    use crate::applications::npm::Npm;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    #[cfg(unix)]
    fn linux_arm() {
      use crate::applications::nodejs::NodeJS;

      let have = (Npm {}).install_methods(
        &Version::from("20.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = vec![Method::ExecutableInAnotherApp {
        other_app: Box::new(NodeJS {}),
        executable_path: S("node-v20.10.0-darwin-arm64/bin/npm"),
      }];
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_intel() {
      let have = (Npm {}).install_methods(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![Method::ExecutableInAnotherApp {
        other_app: Box::new(NodeJS {}),
        executable_path: S("node-v20.10.0-darwin-arm64\\bin\\npm.exe"),
      }];
      assert_eq!(have, want);
    }
  }
}
