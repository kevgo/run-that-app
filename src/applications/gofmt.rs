use super::go::Go;
use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::platform::Platform;
use crate::run::Executable;
use crate::Log;
use crate::{prelude::*, run};

pub struct Gofmt {}

impl App for Gofmt {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("gofmt")
  }

  fn homepage(&self) -> &'static str {
    "https://go.dev"
  }

  fn run_method(&self, _version: &Version, _platform: Platform) -> run::Method {
    run::Method::OtherAppOtherExecutable {
      app: Box::new(app_to_install()),
      executable_name: "gofmt",
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    app_to_install().latest_installable_version(log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    app_to_install().installable_versions(amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("report all errors (not just the first 10 on different lines)") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // TODO: return the version of Go here
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn clone(&self) -> Box<dyn App> {
    Box::new(Gofmt {})
  }
}

fn app_to_install() -> Go {
  Go {}
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::go::Go;
    use crate::applications::gofmt::Gofmt;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    #[cfg(unix)]
    fn macos() {
      use crate::run;

      let have = (Gofmt {}).run_method(
        &Version::from("1.23.4"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = run::Method::OtherAppOtherExecutable {
        app: Box::new(Go {}),
        executable_name: "gofmt",
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows() {
      let have = (Gofmt {}).install_methods(
        &Version::from("1.23.4"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![Method::ExecutableInAnotherApp {
        other_app: Box::new(Go {}),
        executable_path: S("go\\bin\\gofmt.exe"),
      }];
      assert_eq!(have, want);
    }
  }
}
