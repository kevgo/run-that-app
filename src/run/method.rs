use super::{ExecutableArgs, ExecutableNameUnix};
use crate::applications::App;
use crate::configuration::Version;
use crate::installation;
use crate::yard::Yard;

/// the different ways to execute an application
#[derive(Debug, PartialEq)]
pub enum Method {
  /// execute this app's default executable
  ThisApp {
    /// defines the ways in which this app can be installed
    install_methods: Vec<installation::Method>,
  },
  /// executes another executable (not the default executable) of another app
  OtherAppOtherExecutable {
    /// the other application that contains the executable
    app: Box<dyn App>,
    /// name of the executable to run
    executable_name: ExecutableNameUnix,
  },
  /// executes the default executable of another app with additional arguments
  OtherAppDefaultExecutable {
    /// the other applications whose default executable to run
    app: Box<dyn App>,
    /// additional arguments when running the default executable of the given app
    args: ExecutableArgs,
  },
}

impl Method {
  pub fn install_methods(self) -> Vec<installation::Method> {
    match self {
      Method::ThisApp { install_methods } => install_methods,
      Method::OtherAppOtherExecutable { app: _, executable_name: _ } | Method::OtherAppDefaultExecutable { app: _, args: _ } => vec![],
    }
  }

  pub fn call_args(&self, version: &Version, yard: &Yard) -> Option<Vec<String>> {
    match self {
      Method::ThisApp { install_methods: _ } | Method::OtherAppOtherExecutable { app: _, executable_name: _ } => Some(vec![]),
      Method::OtherAppDefaultExecutable { app, args } => match args {
        ExecutableArgs::OneOfTheseInAppFolder { options } => {
          let app_folder = yard.app_folder(&app.name(), version);
          for option in options {
            let full_path = app_folder.join(option);
            if full_path.exists() {
              return Some(vec![full_path.to_string_lossy().to_string()]);
            }
          }
          None
        }
        ExecutableArgs::None => None,
      },
    }
  }
}
