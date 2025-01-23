use super::ExecutableNameUnix;
use crate::applications::App;
use crate::installation;

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
    args: Vec<String>,
  },
}

impl Method {
  pub fn install_methods(self) -> Vec<installation::Method> {
    match self {
      Method::ThisApp { install_methods } => install_methods,
      Method::OtherAppOtherExecutable { app: _, executable_name: _ } | Method::OtherAppDefaultExecutable { app: _, args: _ } => vec![],
    }
  }
}
