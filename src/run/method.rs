use super::ExecutableNameUnix;
use crate::applications::AppDefinition;
use crate::configuration::Version;
use crate::installation::{self, BinFolder};
use crate::yard::Yard;
use std::fmt::{Display, Write};

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

impl Method {
  pub fn install_methods(self) -> Vec<installation::Method> {
    match self {
      Method::ThisApp { install_methods } => install_methods,
      Method::OtherAppOtherExecutable {
        app_definition: _,
        executable_name: _,
      }
      | Method::OtherAppDefaultExecutable { app_definition: _, args: _ } => vec![],
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

/// arguments that are required to execute an application itself - these are not arguments provided by the user
#[derive(Clone, Debug, PartialEq)]
pub enum ExecutableArgs {
  /// the executable can be called without any additional arguments
  None,
  /// uses the first of the given options that exists inside the folder that application is installed in
  OneOfTheseInAppFolder { options: Vec<&'static str> },
}

impl ExecutableArgs {
  /// makes the arguments
  pub fn make_absolute(self, bin_folder: BinFolder) -> Vec<String> {
    match self {
      ExecutableArgs::None => vec![],
      ExecutableArgs::OneOfTheseInAppFolder { options } => {
        for option in options {
          let absolute_path = bin_folder.join(option);
          println!("444444444444444444444 {}", absolute_path.to_string_lossy());
          if absolute_path.exists() {
            println!("exists");
            return vec![absolute_path.to_string_lossy().to_string()];
          }
          println!("doesn't exist");
        }
        vec![]
      }
    }
  }
}

impl Display for ExecutableArgs {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ExecutableArgs::None => f.write_str("no args"),
      ExecutableArgs::OneOfTheseInAppFolder { options } => {
        f.write_str("one of these filesystem entries:")?;
        for option in options {
          f.write_char(' ')?;
          f.write_str(option)?;
        }
        Ok(())
      }
    }
  }
}
