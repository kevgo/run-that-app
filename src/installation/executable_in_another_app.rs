use crate::applications::App;
use crate::configuration::Version;
use crate::platform::Platform;

/// defines the information needed for apps who have a dedicated executable, but this executable is shipped as part of another app
pub trait ExecutableInAnotherApp: App {
  /// the application that ships the executable of this app
  fn app_to_install(&self) -> Box<dyn App>;

  /// location of this app's executable within the archive of the other app
  fn executable_path_in_other_app_yard(&self, version: &Version, platform: Platform) -> String;
}
