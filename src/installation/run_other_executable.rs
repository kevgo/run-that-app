use crate::applications::App;
use crate::configuration::Version;
use crate::platform::Platform;
use crate::subshell::Executable;

/// defines the information needed for apps who execute by running the executable of another application
pub trait RunOtherExecutable {
  /// the application that ships the executable of this app
  fn app_to_install(&self) -> Box<dyn App>;

  /// location of this app's executable within the archive of the other app
  fn call_signature(&self, version: &Version, platform: Platform) -> CallSignature;
}

pub struct CallSignature {
  pub executable: Executable,
  pub args: Vec<String>,
}
