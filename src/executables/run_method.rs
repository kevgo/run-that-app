use super::executable_call::ExecutableArgs;
use super::ExecutableNameUnix;
use crate::applications::AppDefinition;
use crate::installation;

/// the different ways to execute an application
#[derive(Debug, PartialEq)]
pub(crate) enum RunMethod {
  /// execute this app's default executable
  ThisApp {
    /// defines the ways in which this app can be installed
    install_methods: Vec<installation::Method>,
  },
  /// executes another executable (not the default executable) of another app
  OtherAppOtherExecutable {
    /// the other application that contains the executable
    app_definition: Box<dyn AppDefinition>,
    /// name of the executable to run
    executable_name: ExecutableNameUnix,
  },
  /// executes the default executable of another app with additional arguments
  OtherAppDefaultExecutable {
    /// the other applications whose default executable to run
    app_definition: Box<dyn AppDefinition>,
    /// additional arguments when running the default executable of the given app
    args: ExecutableArgs,
  },
}

impl RunMethod {
  pub(crate) fn install_methods(self) -> Vec<installation::Method> {
    match self {
      RunMethod::ThisApp { install_methods } => install_methods,
      RunMethod::OtherAppOtherExecutable {
        app_definition: _,
        executable_name: _,
      }
      | RunMethod::OtherAppDefaultExecutable { app_definition: _, args: _ } => vec![],
    }
  }
}
