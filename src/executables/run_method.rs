use super::ExecutableNameUnix;
use crate::applications::AppDefinition;
use crate::installation;

/// the different ways to execute an application
#[derive(Debug, PartialEq)]
pub enum RunMethod {
  /// execute this app's default executable
  ThisApp {
    /// defines the ways in which this app can be installed
    install_methods: Vec<installation::Method>,
  },

  /// executes another executable (not the default executable) of another app
  OtherAppOtherExecutable {
    /// the other application that contains the executable
    carrier: Box<dyn AppDefinition>,
    /// name of the executable to run
    executable_name: ExecutableNameUnix,
  },

  /// executes a shell script bundled with another app
  OtherAppShellScript {
    /// the other application that contains the shell script
    carrier: Box<dyn AppDefinition>,
    /// name of the shell script to run
    script_name: &'static str,
  },

  /// the app is a `NodeJS` package
  NodeJS {
    /// name of the `NodeJS` package to install
    package: &'static str,

    /// the shell script created by the package to execute
    script: &'static str,
  },
}

impl RunMethod {
  pub fn install_methods(self) -> Vec<installation::Method> {
    match self {
      RunMethod::ThisApp { install_methods } => install_methods,
      RunMethod::NodeJS { package, script } => vec![installation::Method::InstallNodeJSPackage { package, script }],
      RunMethod::OtherAppOtherExecutable {
        carrier: _,
        executable_name: _,
      }
      | RunMethod::OtherAppShellScript { carrier: _, script_name: _ } => vec![],
    }
  }
}
