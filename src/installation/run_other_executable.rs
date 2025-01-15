use crate::applications::App;
use crate::platform::Platform;
use crate::subshell::Executable;

/// defines the information needed for apps who execute by running the executable of another application
pub trait RunOtherExecutable: App {
  /// the application that ships the executable of this app
  fn app_to_execute(&self) -> Box<dyn App>;

  /// provides the name of the executable to call within the app folder of `app_to_execute`
  fn executable_to_call(&self, platform: Platform) -> String;

  /// provides the arguments to call the given executable with
  fn call_signature(&self, executable: Executable) -> CallSignature;
}

pub struct CallSignature {
  pub executable: Executable,
  pub args: Vec<String>,
}
