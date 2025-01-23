use super::ExecutableNameUnix;
use crate::applications::AppDefinition;
use crate::installation;

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
    app: Box<dyn AppDefinition>,
    /// name of the executable to run
    executable_name: ExecutableNameUnix,
  },
  /// executes the default executable of another app with additional arguments
  OtherAppDefaultExecutable {
    /// the other applications whose default executable to run
    app: Box<dyn AppDefinition>,
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
