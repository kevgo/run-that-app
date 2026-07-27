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
    carrier_app: Box<dyn AppDefinition>,
    /// name of the executable to run
    executable_name: ExecutableNameUnix,
  },

  /// executes a shell script bundled with another app
  OtherAppShellScript {
    /// the other application that contains the shell script
    // TODO rename this field to "carier_app" in all variants
    carrier_app: Box<dyn AppDefinition>,
    /// name of the shell script to run
    script_name: &'static str,
  },

  /// the app to run is a `NodeJS` package
  NodeJS {
    /// name of the `NodeJS` package to install
    package: &'static str,
  },
}

impl RunMethod {
  pub fn install_methods(self) -> Vec<installation::Method> {
    match self {
      RunMethod::ThisApp { install_methods } => install_methods,
      RunMethod::NodeJS { package } => vec![installation::Method::InstallNodeJSPackage { package }],
      RunMethod::OtherAppOtherExecutable {
        carrier_app: _,
        executable_name: _,
      }
      | RunMethod::OtherAppShellScript {
        carrier_app: _,
        script_name: _,
      } => vec![],
    }
  }
}
