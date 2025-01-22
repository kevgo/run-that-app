use super::ExecutableNameUnix;
use crate::applications::App;
use crate::configuration::Version;
use crate::installation::{self, BinFolder};
use crate::yard::Yard;
use std::fmt::{Display, Write};
use std::path::Path;

/// the different ways to execute an application
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

impl std::fmt::Debug for Method {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ThisApp { install_methods } => f.debug_struct("ThisApp").field("install_methods", install_methods).finish(),
      Self::OtherAppOtherExecutable { app, executable_name } => f
        .debug_struct("OtherAppOtherExecutable")
        .field("app", &app.name())
        .field("executable_name", executable_name)
        .finish(),
      Self::OtherAppDefaultExecutable { app, args } => f
        .debug_struct("OtherAppDefaultExecutable")
        .field("app", &app.name())
        .field("args", args)
        .finish(),
    }
  }
}

impl PartialEq for Method {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (
        Self::ThisApp {
          install_methods: l_install_methods,
        },
        Self::ThisApp {
          install_methods: r_install_methods,
        },
      ) => l_install_methods == r_install_methods,
      (
        Self::OtherAppOtherExecutable {
          app: l_app,
          executable_name: l_executable_name,
        },
        Self::OtherAppOtherExecutable {
          app: r_app,
          executable_name: r_executable_name,
        },
      ) => l_app.name() == r_app.name() && l_executable_name == r_executable_name,
      (Self::OtherAppDefaultExecutable { app: l_app, args: l_args }, Self::OtherAppDefaultExecutable { app: r_app, args: r_args }) => {
        l_app.name() == r_app.name() && l_args == r_args
      }
      _ => false,
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
